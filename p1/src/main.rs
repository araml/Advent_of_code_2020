use std::fs;

fn main() {
    let txt = fs::read_to_string("input.txt").expect("Error opening file");
    let numbers : Vec<u32> = txt.split("\n").filter(|w| !w.is_empty())
                                .map(|w| w.parse::<u32>().unwrap()).collect();

    for i in &numbers {
        for k in &numbers {
            for j in &numbers {
                if i + k + j == 2020 {
                    println!("Values are: {}, {} and {} \n The result is: {}",
                             i, k, j, j * i * k);
                    return;
                }
            }
        }
    }
}
