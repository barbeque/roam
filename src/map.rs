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
}

pub fn generate_map() -> Dungeon {
    let mut d = Dungeon::new();

    d.set_at(10, 10, ' ');

    d
}