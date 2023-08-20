use std::{
    cmp::max,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut buffer: String = String::new();
    let _bytes_read = io::stdin().read_to_string(&mut buffer);

    let most_cals = calculacte_most_calories(&buffer);

    println!("{}", most_cals);

    Ok(())
}

fn calculacte_most_calories(calories_list: &str) -> i32 {
    let lines = calories_list.split("\n").collect::<Vec<&str>>();

    let mut max_cals: i32 = 0;
    let mut acc: i32 = 0;
    for line in lines {
        match line.parse::<i32>() {
            Err(_) => {
                max_cals = max(acc, max_cals);
                acc = 0;
                continue;
            }
            Ok(cals) => {
                acc += cals;
            }
        }
    }

    return max(acc, max_cals);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_most_calories() {
        let result = calculacte_most_calories(
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n",
        );

        assert_eq!(result, 24000);
    }
}
