use std::fs;


fn main() {
    let txt = fs::read_to_string("../input.txt").expect("Error opening file");

    let mut mat : Vec<Vec<char>> = txt.split('\n').collect::<Vec<&str>>()
                                                  .iter().map(|w| w.chars()
                                                  .collect()).collect();
    mat.pop();

    let width = mat[0].len(); // get len for modulo op
    // we go down one each step..
    let mut x : usize = 3;
    let mut n_trees : u32 = 0;
    for y in 1..mat.len() {
        x = x % width;
        if mat[y][x] == '#' {
            n_trees = n_trees + 1;
        }
        x = x + 3;
    }

    println!("Number of tree encountered: {}", n_trees);
}
