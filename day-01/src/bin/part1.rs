fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!( "{}", output);
}

fn part1(input: &str) -> u32 {
    input.to_string()
    .split("\n\n")
    .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000");
        assert_eq!(result, 24000);
    }
}