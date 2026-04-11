use generator::map_generator;

fn main() {
    let layout = map_generator();
    let mut count: u32 = 0;
    for row in layout.rows() {
        for &val in row {
            let c = match val {
                0 => '.',
                _ => '#',
            };
            if val != 0 {count += 1}
            print!("{} ", c);
        }
        println!();
    }
    println!("{}", count)
}