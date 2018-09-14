pub enum Item {
    Normal(i32), // first value always weight
    Weapon(i32, i32, DamageType), // damage, type
    Food(i32, i32), // recovery hp
}

pub enum DamageType {
    Normal,
    Fire
}
