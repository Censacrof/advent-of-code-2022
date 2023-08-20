mod lib;

use std::{
    cmp::max,
    io::{self, Read},
};

use lib::binary_search_tree::BinaryTree;

fn main() -> io::Result<()> {
    let mut buffer: String = String::new();
    let _bytes_read = io::stdin().read_to_string(&mut buffer);

    let most_cals = calculate_most_calories(&buffer);

    println!("{}", most_cals);

    Ok(())
}

fn calculate_most_calories(calories_list: &str) -> i32 {
    let lines = calories_list.split("\n").collect::<Vec<&str>>();

    let mut max_cals: i32 = 0;
    let mut acc: i32 = 0;
    for line in lines {
        match line.parse::<i32>() {
            Err(_) => {
                max_cals = max(acc, max_cals);
                acc = 0;
            }
            Ok(cals) => {
                acc += cals;
            }
        }
    }

    return max(acc, max_cals);
}

fn calculate_top_3_calories(calories_list: &str) -> (Option<i32>, Option<i32>, Option<i32>) {
    let lines = calories_list.split("\n").collect::<Vec<&str>>();

    let mut tree: Option<BinaryTree> = None;
    let mut acc: i32 = 0;
    for line in lines {
        match line.parse::<i32>() {
            Err(_) => {
                match &mut tree {
                    None => {
                        tree = Some(BinaryTree::new(acc));
                    }
                    Some(t) => t.binary_insert_desc(acc),
                }

                acc = 0
            }
            Ok(cals) => {
                acc += cals;
            }
        }
    }

    match tree {
        Some(t) => {
            let traversed = t.traverse();

            println!("{:?}", traversed);

            (
                traversed.get(0).copied(),
                traversed.get(1).copied(),
                traversed.get(2).copied(),
            )
        }
        None => (None, None, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_most_calories() {
        let result = calculate_most_calories(
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n",
        );

        assert_eq!(result, 24000);
    }

    #[test]
    fn test_calculate_top_3_calories() {
        let result = calculate_top_3_calories(
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n",
        );

        assert_eq!(result, (Some(24000), Some(11000), Some(10000),));
    }
}
