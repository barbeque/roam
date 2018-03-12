pub struct Entity {
    // An entity is any object that is present in the world.
    pub hit_points: i32,
    pub max_hit_points: i32,
    pub location_x: i32,
    pub location_y: i32,
    pub view: char,
}

impl Entity {
    pub fn new() -> Entity {
        // TODO: make this private when we do composition
        Entity {
            hit_points: 10,
            max_hit_points: 10,
            location_x: 0,
            location_y: 0,
            view: '@',
        }
    }
}
