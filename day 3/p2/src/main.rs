use std::fs;

fn find_collisions(map : &Vec<Vec<char>>, x_slope : usize, y_slope : usize) -> usize {
    let width = map[0].len(); // get len for modulo op
    // we go down one each step..
    let mut x : usize = 0;
    let mut n_trees : usize = 0;
    for y in (0..map.len()).step_by(y_slope) {
        x = x % width;
        if map[y][x] == '#' {
            n_trees = n_trees + 1;
        }
        x = x + x_slope;
    }

    return n_trees;
}

fn main() {
    let txt = fs::read_to_string("../input.txt").expect("Error opening file");

    let mut map : Vec<Vec<char>> = txt.split('\n').collect::<Vec<&str>>()
                                                  .iter().map(|w| w.chars()
                                                  .collect()).collect();
    map.pop();
    let t1 = find_collisions(&map, 1, 1);
    let t2 = find_collisions(&map, 3, 1);
    let t3 = find_collisions(&map, 5, 1);
    let t4 = find_collisions(&map, 7, 1);
    let t5 = find_collisions(&map, 1, 2);


    println!("Number of tree encountered: {} {} {} {} {}", t1, t2, t3, t4, t5);
    println!("Number of tree encountered: {}", t1 * t2 * t3 * t4 * t5);
}
