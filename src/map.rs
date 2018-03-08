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
    d.flood_fill(left, top, width, height, ' ');
    true
}

pub fn generate_map() -> Dungeon {
    let mut d = Dungeon::new();
    let mut rng = rand::thread_rng();

    let mut x_range = Range::<usize>::new(1, d.get_width() - 1);
    let mut y_range = Range::<usize>::new(1, d.get_height() - 1);

    let number_of_rooms = 12;

    for _i in 0..number_of_rooms {
        let x = x_range.sample(&mut rng);
        let y = y_range.sample(&mut rng);
        let max_width = (d.get_width() - 1) - x;
        let max_height = (d.get_height() - 1) - y;

        if max_height < 4 || max_width < 4 {
            // FIXME: use a smarter range instead of this hack
            continue;
        }

        let mut width_range = Range::<usize>::new(4, max_width);
        let mut height_range = Range::<usize>::new(4, max_height);
        let room_width = width_range.sample(&mut rng);
        let room_height = height_range.sample(&mut rng);

        generate_room(&mut d, x, y, room_width, room_height);
    }

    d
}
