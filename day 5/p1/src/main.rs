use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_row() {
        assert_eq!(partition_space("FBFBBFF", 'B', 'F', 127), 44);
        assert_eq!(partition_space("BFFFBBF", 'B', 'F', 127), 70);
        assert_eq!(partition_space("FFFBBBF", 'B', 'F', 127), 14);
        assert_eq!(partition_space("BBFFBBF", 'B', 'F', 127), 102);
        assert_eq!(partition_space("FFFFFFF", 'B', 'F', 127), 0);
        assert_eq!(partition_space("BBBBBBB", 'B', 'F', 127), 127);
    }

    #[test]
    fn test_find_column() {
        assert_eq!(partition_space("RLR", 'R', 'L', 7), 5);
        assert_eq!(partition_space("RRR", 'R', 'L', 7), 7);
        assert_eq!(partition_space("RLL", 'R', 'L', 7), 4);
        assert_eq!(partition_space("LLL", 'R', 'L', 7), 0);
    }

    #[test]
    fn test_find_id() {
        let row = partition_space("BFFFBBF", 'B', 'F', 127);
        let column = partition_space("RRR", 'R', 'L', 7);
        assert_eq!(find_id(row, column), 567);

        let row = partition_space("FFFBBBF", 'B', 'F', 127);
        let column = partition_space("RRR", 'R', 'L', 7);
        assert_eq!(find_id(row, column), 119);

        let row = partition_space("BBFFBBF", 'B', 'F', 127);
        let column = partition_space("RLL", 'R', 'L', 7);
        assert_eq!(find_id(row, column), 820);
    }
}

fn find_id(row : u32, column : u32) -> u32 {
    return (row * 8) + column;
}

fn partition_space(bs : &str, upper_comparator : char,
                   lower_comparator: char, initial : u32) -> u32 {
    let mut lower : u32 = 0;
    let mut upper : u32 = initial;
    for i in bs.chars() {
        if i == lower_comparator {
                upper = upper - ((upper - lower) as f64 / 2.0).round() as u32;
        } else if i == upper_comparator {
                lower = lower + ((upper - lower) as f64 / 2.0).round() as u32;
        }
    }

    return upper;
}

fn map_data_to_ids(data : &str) -> u32 {
    let row = partition_space(&data[0..7], 'B', 'F', 127);
    let column = partition_space(&data[7..data.len()], 'R', 'L', 7);
    find_id(row, column)
}

fn main() {
    let txt = fs::read_to_string("../input.txt").expect("Error opening file");

    let max_id : u32 = txt.split_terminator("\n")
                          .map(map_data_to_ids).max().unwrap();

    println!("The highest id is {}", max_id);
}
