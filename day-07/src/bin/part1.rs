fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!( "{}", output);
}

fn part1(input: &str) -> i32 {
    142
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142);
    }
}