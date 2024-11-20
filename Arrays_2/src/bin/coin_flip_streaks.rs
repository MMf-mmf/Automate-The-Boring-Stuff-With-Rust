use rand::Rng;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let mut numberOfStreaks = 0;
    for _ in 0..10000 {
        // Code that creates a list of 100 'heads' or 'tails' values.
        let mut flips = Vec::new();
        for _ in 0..100 {
            let flip = if rand::thread_rng().gen_bool(0.5) {
                "H"
            } else {
                "T"
            };
            flips.push(flip);
        }
        // Code that checks if there is a streak of 6 heads or tails in a row.
        for i in 0..flips.len() - 6 {
            if flips[i..i + 6].iter().all(|&x| x == "H")
                || flips[i..i + 6].iter().all(|&x| x == "T")
            {
                numberOfStreaks += 1;
                break;
            }
        }
    }
    let elapsed_time = start_time.elapsed();
    println!("Chance of streak: {}%", numberOfStreaks / 100);
    println!("Time taken: {:.6} seconds", elapsed_time.as_secs_f64());
}
