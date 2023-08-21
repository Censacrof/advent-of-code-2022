use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer: String = String::new();
    let _bytes_read = io::stdin().read_to_string(&mut buffer);

    println!("{:?}", _bytes_read);

    Ok(())
}

fn calculate_score(moves: &str) -> i32 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_score() {
        let res = calculate_score("A Y\nB X\nC Z\n");
        assert_eq!(res, 15)
    }
}
