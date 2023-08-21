use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer: String = String::new();
    let _bytes_read = io::stdin().read_to_string(&mut buffer);

    println!("{:?}", _bytes_read);

    Ok(())
}

fn get_repeated_char(rucksack: &str) -> char {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_get_repeated_char {
        ($($name:ident($input:expr, $expected:expr);)*) => {
            mod test_get_repeated_char {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        let res = get_repeated_char($input);
                        assert_eq!(res, $expected);
                    }
                )*
            }
        };
    }

    test_get_repeated_char! {
        example_1("vJrwpWtwJgWrhcsFMMfFFhFp", 'p');

        example_2("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L');

        example_3("PmmdzqPrVvPwwTWBwg", 'P');

        example_4("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v');

        example_5("ttgJtRGJQctTZtZT", 't');

        example_6("CrZsJsPPZsGzwwsLwLmpwMDw", 's');
    }
}
