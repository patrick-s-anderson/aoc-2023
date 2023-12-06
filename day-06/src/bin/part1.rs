fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!( "{}", output);
}

fn part1(input: &str) -> i32 {
    288
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
"Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, 288);
    }
}