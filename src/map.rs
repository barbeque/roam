use rand;
use rand::Rng;
use rand::distributions::{Range, Sample};
use std::cmp;

use coordinate_utils::{Rect, overlaps_horizontal, overlaps_vertical, find_overlap_1d};

const MAP_WIDTH: usize = 100;
const MAP_HEIGHT: usize = 100; // probably should be larger, but let's go with it

pub struct Dungeon {
    tiles: Vec<char>,
}
type Room = Rect;

impl Dungeon {
    fn new() -> Dungeon {
        Dungeon {
            tiles: vec!['#'; MAP_WIDTH * MAP_HEIGHT],
        }
    }

    pub fn get_width(self: &Dungeon) -> usize {
        MAP_WIDTH
    }

    pub fn get_height(self: &Dungeon) -> usize {
        MAP_HEIGHT
    }

    pub fn get_at(self: &Dungeon, x: usize, y: usize) -> char {
        self.tiles[y * MAP_WIDTH + x]
    }

    pub fn set_at(self: &mut Dungeon, x: usize, y: usize, val: char) {
        self.tiles[y * MAP_WIDTH + x] = val;
    }

    pub fn flood_fill(
        self: &mut Dungeon,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
        val: char,
    ) {
        for x in 0..width {
            for y in 0..height {
                self.set_at(left + x, top + y, val);
            }
        }
    }

    pub fn find_room_tile(self: &Dungeon) -> (i32, i32) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if self.get_at(x, y) == '.' {
                    return (x as i32, y as i32);
                }
            }
        }
        (0, 0)
    }
}

fn generate_room(d: &mut Dungeon, left: usize, top: usize, width: usize, height: usize) -> bool {
    // Check if a room already exists there
    for x in 0..width {
        for y in 0..height {
            if d.get_at(left + x, top + y) != '#' {
                // Bail on this location
                return false;
            }
        }
    }

    // Generate the room with a simple flood fill
    d.flood_fill(left, top, width, height, '.');
    true
}

fn generate_hallway_eastwest(x1: i32, x2: i32, y: i32, dungeon: &mut Dungeon) {
    // TODO: write a bresenham
    let start = cmp::min(x1, x2);
    let finish = cmp::max(x1, x2);
    for x in start..(finish + 1) {
        dungeon.set_at(x as usize, y as usize, '.');
    }
}

fn generate_hallway_northsouth(y1: i32, y2: i32, x: i32, dungeon: &mut Dungeon) {
    // TODO: write a bresenham
    let start = cmp::min(y1, y2);
    let finish = cmp::max(y1, y2);
    for y in start..(finish + 1) {
        dungeon.set_at(x as usize, y as usize, '.');
    }
}

pub fn generate_map() -> Dungeon {
    let mut d = Dungeon::new();
    let mut rng = rand::thread_rng();

    let mut x_range = Range::<usize>::new(1, d.get_width() - 1);
    let mut y_range = Range::<usize>::new(1, d.get_height() - 1);

    let number_of_rooms = 35;
    let minimum_room_size = 4;

    let mut rooms = Vec::<Room>::new();

    for _i in 0..number_of_rooms {
        let x = x_range.sample(&mut rng);
        let y = y_range.sample(&mut rng);
        let max_width = (d.get_width() - 1) - x;
        let max_height = ((d.get_height() - 1) - y) / 2;

        if max_height <= minimum_room_size || max_width <= minimum_room_size {
            // FIXME: use a smarter range instead of this hack
            continue;
        }

        let mut width_range = Range::<usize>::new(minimum_room_size, max_width);
        let mut height_range = Range::<usize>::new(minimum_room_size, max_height);

        let room_width = width_range.sample(&mut rng);
        let room_height = height_range.sample(&mut rng);

        if generate_room(&mut d, x, y, room_width, room_height) {
            // Track for later, so we can draw hallways
            rooms.push(Room {
                x: x as i32,
                y: y as i32,
                width: room_width as i32,
                height: room_height as i32,
            });
        }
    }

    let mut room_connections = Vec::<(&Room, &Room)>::new();

    // sketch some hallways between rooms, because why not?
    let hallways = 20;
    for _i in 0..hallways {
        let room_a = rng.choose(&rooms).unwrap();
        let room_b = rng.choose(&rooms).unwrap();

        if room_a == room_b {
            continue; // bail on this one
        }

        if overlaps_vertical(room_a.y, room_a.height, room_b.y, room_b.height) {
            // parallel: east-west
            let (start_y, range_y) = find_overlap_1d(room_a.y, room_a.height, room_b.y, room_b.height);
            let y = Range::<i32>::new(start_y, start_y + range_y).sample(&mut rng);
            generate_hallway_eastwest(room_a.centre().0, room_b.centre().0, y, &mut d);

            room_connections.push((&room_a, &room_b));
        }
        else if overlaps_horizontal(room_a.x, room_a.width, room_b.x, room_b.width) {
            // parallel: north-south
            let (start_x, range_x) = find_overlap_1d(room_a.x, room_a.width, room_b.x, room_b.width);
            let x = Range::<i32>::new(start_x, start_x + range_x).sample(&mut rng);
            generate_hallway_northsouth(room_a.centre().1, room_b.centre().1, x, &mut d);

            room_connections.push((&room_a, &room_b));
        }
    }

    // Now we have all room connections in room_connections,
    // so we know which rooms are isolated and should not have
    // important stuff in them
    let isolated_rooms = find_isolated_rooms(&rooms, &room_connections);

    d
}

pub fn find_isolated_rooms<'a>(all_rooms: &'a Vec<Room>, connections: &Vec<(&Room, &Room)>) -> Vec<&'a Room> {
    all_rooms
        .iter()
        .filter(|&room| connections.iter().any(|&(l, r)| l != room && r != room))
        .collect()
}

#[cfg(test)]
mod dungeon_tests {
    use map::{Dungeon, generate_hallway_eastwest, generate_hallway_northsouth};
    #[test]
    fn dimensions_are_nonzero() {
        let dungeon = Dungeon::new();
        assert!(dungeon.get_width() > 0);
        assert!(dungeon.get_height() > 0);
    }

    #[test]
    fn get_at_set_at_works() {
        let mut dungeon = Dungeon::new();
        assert_eq!(dungeon.get_at(0, 0), '#');
        dungeon.set_at(0, 0, '&');
        assert_eq!(dungeon.get_at(0, 0), '&');
    }

    #[test]
    fn generate_hallways_east_west_works() {
        let mut dungeon = Dungeon::new();
        generate_hallway_eastwest(25, 50, 10, &mut dungeon);
        assert_eq!(dungeon.get_at(24, 10), '#');
        assert_eq!(dungeon.get_at(25, 10), '.');
        assert_eq!(dungeon.get_at(50, 10), '.');
        assert_eq!(dungeon.get_at(51, 10), '#'); // Make sure it didn't run off the end
        for x in 0..dungeon.get_width() {
            // Make sure it didn't draw any hallways above or below
            assert_eq!(dungeon.get_at(x, 9), '#');
            assert_eq!(dungeon.get_at(x, 11), '#');
        }
    }

    #[test]
    fn generate_hallways_north_south_works() {
        let mut dungeon = Dungeon::new();
        generate_hallway_northsouth(25, 50, 10, &mut dungeon);
        assert_eq!(dungeon.get_at(10, 24), '#');
        assert_eq!(dungeon.get_at(10, 25), '.');
        assert_eq!(dungeon.get_at(10, 50), '.');
        assert_eq!(dungeon.get_at(10, 51), '#'); // Make sure it didn't run off the end
        for y in 0..dungeon.get_height() {
            // Make sure it didn't draw any hallways west or east
            assert_eq!(dungeon.get_at(9, y), '#');
            assert_eq!(dungeon.get_at(11, y), '#');
        }
    }

    #[test]
    fn flood_fill_works() {
        let mut dungeon = Dungeon::new();

        dungeon.flood_fill(10, 10, 10, 10, '$');
        for x in 10..20 {
            for y in 10..20 {
                assert_eq!(dungeon.get_at(x, y), '$');
            }
        }

        assert_eq!(dungeon.get_at(0, 0), '#');
        assert_eq!(dungeon.get_at(9, 10), '#');
        assert_eq!(dungeon.get_at(20, 20), '#');
    }

    #[test]
    fn isolated_room_detection_works() {
        use map::Room;
        use map::find_isolated_rooms;

        let room_a = Room { x: 10, y: 15, width: 5, height: 7 };
        let room_b = Room { x: 10, y: 25, width: 5, height: 7 };
        let room_c = Room { x: 20, y: 25, width: 3, height: 10 };
        let all_rooms = vec![room_a, room_b, room_c];
        let connected_rooms = vec![(&all_rooms[0], &all_rooms[1])];
        let isolated_rooms = find_isolated_rooms(&all_rooms, &connected_rooms);
        assert_eq!(isolated_rooms.len(), 1);

        // Make sure it's room C
        assert_eq!(&all_rooms[2], isolated_rooms[0]);
    }
}
