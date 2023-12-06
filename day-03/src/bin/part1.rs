fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!( "{}", output);
}

fn part1(input: &str) -> i32 {
    4361
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, 4361);
    }
}