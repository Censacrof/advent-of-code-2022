use std::{
    collections::{btree_set::Intersection, HashSet},
    io::{self, Read},
    ops::Deref,
};

fn main() -> io::Result<()> {
    let mut buffer: String = String::new();
    let _bytes_read = io::stdin().read_to_string(&mut buffer);

    println!("{:?}", _bytes_read);

    Ok(())
}

fn get_repeated_char(rucksack: &str) -> char {
    let chars = rucksack.chars().collect::<Vec<char>>();

    let (lower, higher) = chars.split_at(chars.len() / 2);

    let mut a: HashSet<char, _> = HashSet::new();
    for c in lower {
        a.insert(c.clone());
    }

    let mut b: HashSet<char, _> = HashSet::new();
    for c in higher {
        b.insert(c.clone());
    }

    let mut intersection = a.intersection(&b);

    return intersection.next().unwrap().clone();
}

fn get_char_priority(c: char) -> u32 {
    let ascii = c as u32;
    println!("{}", ascii);

    if ascii >= 'a' as u32 {
        return ascii - 'a' as u32 + 1;
    }

    return ascii - 'A' as u32 + (b'z' - b'a') as u32 + 2;
}

fn get_total_priority(input: &str) -> u32 {
    input.lines().fold(0, |acc, rucksack| {
        get_char_priority(get_repeated_char(rucksack)) + acc
    })
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_get_repeated_char {
        use super::*;

        macro_rules! test_get_repeated_char {
            ($($name:ident: $input:expr => $expected:expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let res = get_repeated_char($input);
                        assert_eq!(res, $expected);
                    }
                )*
            };
        }

        test_get_repeated_char! {
            example_1: "vJrwpWtwJgWrhcsFMMfFFhFp" => 'p'
            example_2: "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL" => 'L'
            example_3: "PmmdzqPrVvPwwTWBwg" => 'P'
            example_4: "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn" => 'v'
            example_5: "ttgJtRGJQctTZtZT" => 't'
            example_6: "CrZsJsPPZsGzwwsLwLmpwMDw" => 's'
        }
    }

    mod test_get_char_priority {
        use super::*;

        macro_rules! test_get_char_priority {
            ($($name:ident: $input:expr => $expected:expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let res = get_char_priority($input);
                        assert_eq!(res, $expected);
                    }
                )*
            };
        }

        test_get_char_priority! {
            priority_a: 'a' => 1
            priority_z: 'z' => 26
            priority_upper_a: 'A' => 27
            priority_upper_z: 'Z' => 52
        }
    }

    #[test]
    fn test_get_total_priority() {
        let res = get_total_priority("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n");
        assert_eq!(res, 157);
    }
}
