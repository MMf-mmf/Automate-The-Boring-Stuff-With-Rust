use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let picture_grid = vec![
        vec!['.', '.', '.', '.', '.', '.'],
        vec!['.', 'O', 'O', '.', '.', '.'],
        vec!['O', 'O', 'O', 'O', '.', '.'],
        vec!['O', 'O', 'O', 'O', 'O', '.'],
        vec!['.', 'O', 'O', 'O', 'O', 'O'],
        vec!['O', 'O', 'O', 'O', 'O', '.'],
        vec!['O', 'O', 'O', 'O', '.', '.'],
        vec!['.', 'O', 'O', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.'],
    ];

    let width = picture_grid[0].len();
    let height = picture_grid.len();

    let mut result = String::with_capacity(width * (height + 1));

    for col in 0..width {
        for row in &picture_grid {
            result.push(row[col]);
        }
        result.push('\n');
    }
    let elapsed_time = start_time.elapsed();
    print!("{}", result);

    println!("Time taken: {:.6} seconds", elapsed_time.as_secs_f64());
}
