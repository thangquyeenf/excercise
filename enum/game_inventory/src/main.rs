enum Item {
    Weapon { name: String, damage: u32 },
    Potion { name: String, heal: u32 },
    Gold(u32),
}

fn total_gold(items: &Vec<Item>) -> u32 {
    items
        .iter()
        .map(|x| match x {
            Item::Gold(v) => *v,
            _ => 0,
        })
        .sum()
}

fn use_item(item: &Item) {
    match item {
        Item::Weapon { name, damage } => {
            println!("Player used weapon {} with {} damage", name, damage)
        }
        Item::Potion { name, heal } => println!("Player used {} - {}", name, heal),
        Item::Gold(v) => println!("Player used {} gold", v),
    }
}

fn main() {
    let inventory = vec![
        Item::Weapon {
            name: "Sword".to_string(),
            damage: 15,
        },
        Item::Potion {
            name: "Health Potion".to_string(),
            heal: 25,
        },
        Item::Gold(100),
        Item::Gold(50),
    ];

    println!("Total gold: {}", total_gold(&inventory));

    for item in &inventory {
        use_item(item);
    }
}
