use rand;
use rand::distributions::{Sample, Range};

const MAP_WIDTH : usize = 100;
const MAP_HEIGHT : usize = 100; // probably should be larger, but let's go with it

pub struct Dungeon {
    tiles: Vec<char>,
}

impl Dungeon {
    fn new() -> Dungeon {
        Dungeon {
            tiles: vec!['#'; MAP_WIDTH * MAP_HEIGHT]
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

    pub fn flood_fill(self: &mut Dungeon, left: usize, top: usize, width: usize, height: usize, val: char) {
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
                return false
            }
        }
    }

    // Generate the room with a simple flood fill
    d.flood_fill(left, top, width, height, '.');
    true
}

struct Room {
    left: usize,
    top: usize,
    width: usize,
    height: usize
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
        let max_height = (d.get_height() - 1) - y;

        if max_height <= minimum_room_size || max_width <= minimum_room_size {
            // FIXME: use a smarter range instead of this hack
            continue;
        }

        let mut width_range = Range::<usize>::new(minimum_room_size, max_width);
        let mut height_range = Range::<usize>::new(minimum_room_size, max_height);

        // FIXME: i feel like rooms should be 'wider' and not 'taller' in general
        // to be more rogue-ish
        let room_width = width_range.sample(&mut rng);
        let room_height = height_range.sample(&mut rng);

        if generate_room(&mut d, x, y, room_width, room_height) {
            // Track for later, so we can draw hallways
            rooms.push(Room { left: x, top: y, width: room_width, height: room_height });
        }
    }

    d
}
