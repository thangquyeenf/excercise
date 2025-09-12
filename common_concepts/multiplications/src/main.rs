fn multiplications_table() {
    for i in 1..=9 {
        for j in 1..=9 {
            print!("{:2} ", i * j);
        }
        println!();
    }
}

fn main() {
    multiplications_table();
}
