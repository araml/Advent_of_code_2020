use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search() {
        assert_eq!(find_row("FBFBBFF"), 44);
        assert_eq!(find_row("BFFFBBF"), 70);
        assert_eq!(find_row("FFFBBBF"), 14);
        assert_eq!(find_row("BBFFBBF"), 102);
        assert_eq!(find_row("FFFFFFF"), 0);
        assert_eq!(find_row("BBBBBBB"), 127);
    }
}

fn find_row(bs : &str) -> u32 {
    let mut lower : u32 = 0;
    let mut upper : u32 = 127;
    for i in bs.chars() {
        if i == 'F' {
                upper = upper - ((upper - lower) as f64 / 2.0).round() as u32;
        } else if i == 'B' {
                lower = lower + ((upper - lower) as f64 / 2.0).round() as u32;
        }
    }

    println!("{} - {}", upper, lower);
    return upper;
}

fn main() {
    let txt = fs::read_to_string("../input.txt").expect("Error opening file");

    let mut positions : Vec<&str> = txt.split_terminator("\n").collect::<Vec<&str>>();

}
