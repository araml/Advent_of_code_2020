use std::env;
use std::fs;

fn remove_duplicates(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let copy: Vec<Vec<char>> = data
        .iter()
        .map(|w| {
            let mut v = w.to_vec();
            v.sort();
            v.dedup();
            return v;
        })
        .collect::<Vec<_>>();
    return copy;
}

fn parse_into_chars(input: String) -> Vec<Vec<char>> {
    let parsed = input.split_terminator("\n\n").collect::<Vec<&str>>();
    return parsed
        .iter()
        .map(|x| {
            x.split_terminator("\n")
                .collect::<String>()
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
}

fn sum(filtered : Vec<Vec<char>>) -> usize {
    let mut result : usize = 0;
    for i in filtered {
        result += i.len();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_data() {
        let data: Vec<Vec<char>> = vec![
            vec!['a', 'b', 'c'],
            vec!['a', 'b', 'c'],
            vec!['a', 'b', 'a', 'c'],
            vec!['a', 'a', 'a', 'a'],
            vec!['b'],
        ];
        let sorted_deduped = remove_duplicates(data);

        assert_eq!(sorted_deduped[0], vec!['a', 'b', 'c']);
        assert_eq!(sorted_deduped[1], vec!['a', 'b', 'c']);
        assert_eq!(sorted_deduped[2], vec!['a', 'b', 'c']);
        assert_eq!(sorted_deduped[3], vec!['a']);
        assert_eq!(sorted_deduped[4], vec!['b']);
    }

    #[test]
    fn test_parse_data() {
        let data = String::from("abc\n\nabc\n\nabac\n\naaaa\n\nb");
        let vcvc = parse_into_chars(data);
        assert_eq!(vcvc[0], vec!['a', 'b', 'c']);
        assert_eq!(vcvc[2], vec!['a', 'b', 'a', 'c']);
    }

    #[test]
    fn solution() {
        let data = String::from("abc\n\nabc\n\nabac\n\naaaa\n\nb");
        let vcvc = parse_into_chars(data);
        let filtered = remove_duplicates(vcvc);
        let result = sum(filtered);

        assert_eq!(result, 11);
    }
}

fn find_id(row: u32, column: u32) -> u32 {
    return (row * 8) + column;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let txt = fs::read_to_string("../input.txt").expect("Error opening file");
    if args.len() != 2 {
        panic!(
            "Wrong number of inputs, must provide which part is being solved
                'p1' or 'p2'"
        );
    }

    if args[1] == "p1" {
        let into_chars = parse_into_chars(txt);
        let undupped = remove_duplicates(into_chars);
        let result = sum(undupped);
        println!("The number of counts is {}", result);
    } else if args[1] == "p2" {
    } else {
        println!("Choose the correct part 'p1/p2'");
    }
}
