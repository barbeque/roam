pub enum Item {
    Junk{ weight: u32 }, // first value always weight
    Weapon{ weight: u32, damage: u32, damage_type: DamageType }, // damage, type
    Food { weight: u32, recovery_hp: u32 }, // recovery hp
}

// TODO: Eh how are we going to do item names? This seems like it might need almost like a composition approach.

pub enum DamageType {
    Normal,
    Fire
}

pub struct Inventory {
    items: Vec<Item>
}

pub fn get_weight_of_item(i: &Item) -> u32 {
    // there has to be an easier way to do this. maybe macros?
    match i {
        Item::Junk { weight } => weight.clone(),
        Item::Weapon { weight, .. } => weight.clone(),
        Item::Food { weight, .. } => weight.clone()
    }
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory { items: Vec::<Item>::new() }
    }

    pub fn total_weight(&self) -> u32 {
        self.items.iter().fold(0, |sum, i| sum + get_weight_of_item(i))
    }
}

#[cfg(test)]
mod item_tests {
    use item::{get_weight_of_item, Item};

    #[test]
    fn item_weight_works() {
        let j = Item::Junk { weight: 66 };
        assert_eq!(get_weight_of_item(&j), 66);
    }
}

#[cfg(test)]
mod inventory_tests {
    use item::{Inventory, Item};

    #[test]
    fn inventory_starts_with_no_items() {
        let i = Inventory::new();
        assert_eq!(i.items.len(), 0);
    }

    #[test]
    fn total_weight_works() {
        let mut i = Inventory::new();
        let j = Item::Junk { weight: 66 };
        i.items.push(j);
        assert_eq!(i.total_weight(), 66);
        let k = Item::Junk { weight: 131 };
        i.items.push(k);
        assert_eq!(i.total_weight(), 197);
    }
}
