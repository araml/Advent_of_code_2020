use std::fs;
use std::collections::BTreeMap;

fn check_valid_passport(passport : &Vec<&str>) -> bool {
    let mut pointers : BTreeMap<&str, fn (&str) -> usize> = BTreeMap::new();
    pointers.insert("byr", | x : &str | 2);
    pointers.insert("iyr", | x : &str | 3 );
    pointers.insert("eyr", | x : &str | 5);
    pointers.insert("hgt", | x : &str | 7);
    pointers.insert("hcl", | x : &str | 11);
    pointers.insert("ecl", | x : &str | 13);
    pointers.insert("pid", | x : &str | 17);
    pointers.insert("cid", | x : &str | 19);

    if (passport.len() <= 6) {
        return false;
    }

    let mut result = 1;
    for e in passport {
        match pointers.get(&e[0..3]) {
            None => { }
            Some(v) => {
                         result = result * v(&e[0..3]);
                       }//if !v(&e[5..e.len()]) { return false }
        };
    }

    if result == 9699690 || result == 510510 {
        return true;
    }
    return false;
}

fn compact(input : &str) -> Vec<&str> {
    let mut result : Vec<&str> = Vec::new();
    let input = input.split_terminator("\n").collect::<Vec<&str>>();
    for i in input {
        let mut c = i.split(" ").collect::<Vec<&str>>();
        result.append(&mut c);
    }

    return result;
}

fn main() {
    let mut txt = fs::read_to_string("../input.txt").expect("Error opening file");

    let mut passports : Vec<&str> = txt.split("\n\n").collect::<Vec<&str>>();
    println!("Total passports {}", passports.len());

    let passports : Vec<Vec<&str>> = passports.iter().map(|w| compact(w)).collect::<Vec<Vec<&str>>>();


    println!("Passports after fmt {}", passports.len());

    for i in &passports {
        for k in i {
            println!("{}", k);
        }
        println!("\n\n");
    }
    // we get a vector of passports (Vec<&str>)


    let mut valid_passports : usize = 0;
    for s in passports {
        if s.len() > 0 {
            if check_valid_passport(&s) {
                valid_passports = valid_passports + 1;
            } else {
                println!("{}", s.len());
                for e in s {
                    print!("{} ", e);
                }
                println!("");
            }
        }
    }

    println!("Number of valid passports: {}", valid_passports);

}
