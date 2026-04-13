use ndarray::{Array2};

use generator::map_generator;

fn main() {
    let layout: Array2<u8> = map_generator();
    let mut count: u32 = 0;
    for row in layout.rows() {
        for &val in row {
            let c: char = match val {
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

// fn main() {
//     let mut count: u32 = 0;
//     for _ in 0..10000 {
//         let layout: Array2<u8> = map_generator();
//         for row in layout.rows() {
//             for &val in row {
//                 if val != 0 {count += 1}
//             }
//         }
//     }
//     println!("{}", count/10000)
// }