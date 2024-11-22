use std::collections::HashMap;
use std::time::Instant;

fn display_inventory(inventory: &HashMap<String, i32>) {
    println!("Inventory:");
    for (item, count) in inventory {
        println!("{}: {}", item, count);
    }
    let total_items: i32 = inventory.values().sum();
    println!("Total number of items: {}", total_items);
}

fn add_inventory(inventory: &mut HashMap<String, i32>, loot: &[&str]) {
    for item in loot {
        let count = inventory.entry(item.to_string()).or_insert(0);
        *count += 1;
    }
}

fn main() {
    let start_time = Instant::now();
    let mut inventory = HashMap::from([
        ("rope".to_string(), 1),
        ("torch".to_string(), 6),
        ("gold coin".to_string(), 42),
        ("dagger".to_string(), 1),
        ("arrow".to_string(), 12),
    ]);
    const DRAGON_LOOT: [&str; 5] = ["gold coin", "dagger", "gold coin", "gold coin", "ruby"];
    display_inventory(&inventory);
    add_inventory(&mut inventory, &DRAGON_LOOT);
    display_inventory(&inventory);

    let elapsed_time = start_time.elapsed();

    println!("Time taken: {:.6} seconds", elapsed_time.as_secs_f64());
}
