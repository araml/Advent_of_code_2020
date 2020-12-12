use std::collections::BTreeMap;
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

fn parse_into_chars(input: &str) -> Vec<char> {
    return input
        .split_terminator("\n")
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>();
}

fn sum(filtered: Vec<Vec<char>>) -> usize {
    let mut result: usize = 0;
    for i in filtered {
        result += i.len();
    }
    result
}

struct group {
    n_people: usize,
    apparitions: BTreeMap<char, usize>,
}

fn count_terminators(slice: &str) -> usize {
    let mut result: usize = 1;
    for i in slice.chars() {
        if i == '\n' {
            result = result + 1;
        }
    }
    return result;
}

fn build_group(slice: &str) -> group {
    group {
        n_people: count_terminators(slice),
        apparitions: build_map(&parse_into_chars(slice)),
    }
}

fn build_map(answers: &Vec<char>) -> BTreeMap<char, usize> {
    let mut result: BTreeMap<char, usize> = BTreeMap::new();
    for i in answers.iter() {
        match result.get_mut(&i) {
            Some(mut v) => *v = *v + 1 as usize,
            None => {
                result.insert(*i, 1);
            }
        };
    }
    return result;
}

fn group_to_answers(g: &group) -> u32 {
    return g.apparitions.iter().fold(0, |rec, a| {
        if *a.1 == g.n_people {
            return rec + 1;
        } else {
            return rec;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_to_number_of_answers() {
        let data = String::from("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb");
        let groups = data
            .split_terminator("\n\n")
            .map(|x| build_group(x))
            .collect::<Vec<group>>();

        assert_eq!(group_to_answers(&groups[0]), 3);
        assert_eq!(group_to_answers(&groups[1]), 0);
        assert_eq!(group_to_answers(&groups[2]), 1);
        assert_eq!(group_to_answers(&groups[3]), 1);
        assert_eq!(group_to_answers(&groups[4]), 1);
    }

    #[test]
    fn test_group_to_number_of_answers2() {
        let data = String::from("ke\nke\nek\n\nm\nc");
        let groups = data
            .split_terminator("\n\n")
            .map(|x| build_group(x))
            .collect::<Vec<group>>();

        assert_eq!(group_to_answers(&groups[0]), 2);
        assert_eq!(group_to_answers(&groups[1]), 0);
    }

    #[test]
    fn test_build_groups() {
        let data = String::from("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb");

        let groups = data
            .split_terminator("\n\n")
            .map(|x| build_group(x))
            .collect::<Vec<group>>();

        let mut mp: BTreeMap<char, usize> = BTreeMap::new();
        mp.insert('a', 1);
        mp.insert('b', 1);
        mp.insert('c', 1);

        assert_eq!(groups[0].n_people, 1);
        assert_eq!(groups[0].apparitions, mp);
    }

    #[test]
    fn test_build_map() {
        let m1 = build_map(&vec!['a', 'b', 'c']);
        let m2 = build_map(&vec!['a', 'a', 'a', 'a']);

        assert_eq!(*m1.get(&'a').unwrap(), 1);
        assert_eq!(*m2.get(&'a').unwrap(), 4);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut txt = fs::read_to_string("../input.txt").expect("Error opening file");

    if (txt.as_bytes()[txt.len() - 1] == '\n' as u8) {
        println!("remove new_line");
        txt.pop();
    }

    if args.len() != 2 {
        panic!(
            "Wrong number of inputs, must provide which part is being solved
                'p1' or 'p2'"
        );
    }

    if args[1] == "p1" {
        //   let into_chars = parse_into_chars(txt);
        //    let undupped = remove_duplicates(into_chars);
        //    let result = sum(undupped);
        //     println!("The number of counts is {}", result);
    } else if args[1] == "p2" {
        let groups = txt
            .split_terminator("\n\n")
            .map(|x| build_group(x))
            .collect::<Vec<group>>();
        let answers = groups
            .iter()
            .map(|x| group_to_answers(x))
            .collect::<Vec<u32>>();
        let mut answer = 0;
        for i in answers {
            answer = answer + i;
        }
        println!("{}", answer);
    } else {
        println!("Choose the correct part 'p1/p2'");
    }
}
