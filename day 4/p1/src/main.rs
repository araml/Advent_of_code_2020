use std::fs;
use std::collections::BTreeMap;

fn check_valid_passport(passport : Vec<&str>) -> bool {
    let mut pointers : BTreeMap<&str, Box<dyn Fn (&str) -> bool>> = BTreeMap::new();
    pointers.insert("byr", Box::new(| x : &str | x.len() > 5));
    pointers.insert("iyr", Box::new(| x : &str | x.len() > 5));
    pointers.insert("eyr", Box::new(| x : &str | x.len() > 5));
    pointers.insert("hgt", Box::new(| x : &str | x.len() > 5));
    pointers.insert("hcl", Box::new(| x : &str | x.len() > 5));
    pointers.insert("ecl", Box::new(| x : &str | x.len() > 5));
    pointers.insert("pid", Box::new(| x : &str | x.len() > 5));
    pointers.insert("cid", Box::new(| x : &str | x.len() > 5));

    for e in passport {
        match pointers.get(&e[0..3]) {
            None => if e[5..e.len()] != *"cid" { return false; }
            Some(v) => if !v(&e[5..e.len()]) { return false }
        };
    }
    return true;
}

fn main() {
    let txt = fs::read_to_string("../input.txt").expect("Error opening file");

    let mut passports : Vec<Vec<&str>> = txt.split("\n\n")
                                      .map(|w| w.split(" ").collect::<Vec<_>>())
                                      .collect::<Vec<_>>();
    passports.pop();

    // we get a vector of passports (Vec<&str>)

    let mut valid_passports : usize = 0;
    for s in passports {
        if check_valid_passport(s) {
            valid_passports = valid_passports + 1;
        }
    }


    println!("Number of valid passports: {}", valid_passports);
}
