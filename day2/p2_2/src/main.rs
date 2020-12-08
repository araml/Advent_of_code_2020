use std::fs;

fn check_pw(pattern : Vec<usize>, letter : char, password : &str) -> bool {
    let slice = &password[1..];
    let beg : usize = pattern[0];
    let end : usize = pattern[1];
    let appearances : Vec<char> = slice.chars().collect();
    if appearances.len() >= beg && appearances.len() >= end {
        let possible_letters = vec![appearances[beg - 1], appearances[end - 1]];
        let n_letters : usize = possible_letters.iter().filter(|l| !(**l == letter)).count();
        return n_letters == 1;
    }
    return false;
}

fn main() {
    let txt = fs::read_to_string("input.txt").expect("Error opening file");
    let mut possible_pws : Vec<&str> = txt.split("\n").collect();
    possible_pws.pop();

    let mut wrong_pws : u32 = 0;
    for possible in &possible_pws {
        // we have number-number letter: password, we split the first two
        let poss_split : Vec<&str> = possible.split(":").collect();
        // we split the numbers from the letter
        let pattern : Vec<&str> = poss_split[0].split(" ").collect();
        let numbers_in_pattern : Vec<usize> = pattern[0].split("-")
                                                        .map(|w| w.parse::<usize>()
                                                        .unwrap())
                                                        .collect();
        let pw : &str = poss_split[1];
        if check_pw(numbers_in_pattern, pattern[1].parse::<char>().unwrap(), pw) {
            wrong_pws = wrong_pws + 1;
        }
    }

    println!("Number of wrong pws: {}", wrong_pws);
}
