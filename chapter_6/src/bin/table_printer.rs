use std::time::Instant;

fn print_table(table_data: [[&str; 4]; 3]) {
    let mut col_widths = vec![0; 4];
    // loop over the table_data to get the max width of every col
    for (i, inner_list) in table_data.iter().enumerate() {
        col_widths[i] = inner_list.iter().map(|s| s.len()).max().unwrap();
    }
    // println!("{:?}", col_width);
    // now we will loop over the rows and print each line
    let num_rows = table_data[0].len();
    for row in 0..num_rows {
        for col in 0..table_data.len() {
            print!("{:>width$} ", table_data[col][row], width = col_widths[col]);
        }
        println!("")
    }
}

fn main() {
    let start_time = Instant::now();
    const TABLE_DATA: [[&str; 4]; 3] = [
        ["apples", "oranges", "cherries", "banana"],
        ["Alice", "Bob", "Carol", "David"],
        ["dogs", "cats", "moose", "goose"],
    ];
    print_table(TABLE_DATA);

    let elapsed_time = start_time.elapsed();
    println!("Time taken: {:.6} seconds", elapsed_time.as_secs_f64());
}
