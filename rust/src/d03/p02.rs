use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut buffer: String = String::new();
    let _bytes_read = io::stdin().read_to_string(&mut buffer);

    println!("{:?}", _bytes_read);

    Ok(())
}

macro_rules! intersect (
    ($a:expr, $b:expr) => {
        {
            let mut a: HashSet<char, _> = HashSet::new();
            for c in $a.chars() {
                a.insert(c.clone());
            }

            let mut b: HashSet<char, _> = HashSet::new();
            for c in $b.chars() {
                b.insert(c.clone());
            }

            let asd = a.intersection(&b);
            Box::new(asd.clone())
        }
    };
);

fn get_char_priority(c: char) -> u32 {
    let ascii = c as u32;

    if ascii >= 'a' as u32 {
        return ascii - 'a' as u32 + 1;
    }

    return ascii - 'A' as u32 + (b'z' - b'a') as u32 + 2;
}

#[cfg(test)]
mod test {
    use super::*;

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

    mod test_intersect {
        use super::*;

        fn two_args() {
            let mut res = {
                let mut a: HashSet<char, _> = HashSet::new();
                for c in "hello".chars() {
                    a.insert(c.clone());
                }

                let mut b: HashSet<char, _> = HashSet::new();
                for c in "hello".chars() {
                    b.insert(c.clone());
                }

                let asd = a.intersection(&b);
                Box::new(asd.clone())
            };

            // let mut res = intersect!("hello", "world");

            // assert_eq!(res.find(|&&c| c == 'h').clone(), None);
            // assert_eq!(res.find(|&&c| c == 'e').clone(), None);
            // assert_eq!(res.find(|&&c| c == 'l').clone(), None);
            // assert_eq!(res.find(|&&c| c == 'o').clone(), None);
            // assert_eq!(res.find(|&&c| c == 'w').clone(), None);
            // assert_eq!(res.find(|&&c| c == 'r').clone(), None);
            // assert_eq!(res.find(|&&c| c == 'd').clone(), None);
        }
    }
}
