fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    println!( "{}", output);
}

fn part2(input: &str) -> i32 {
    let mut v1 = vec![];
    let mut v2 = vec![];

    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut result = 0;
    for line in input.to_string().lines() {
        let owned = line.to_owned();
        let index = numbers.iter().position(|&r| r == owned);
        match index {
            Some(index) => println!("{}", index),
            None => println!("none"),

        }
        for c in line.chars() { 
            if c.is_numeric() {
                v1.push(c);
                break;
            }
        }
  }

  for line in input.to_string().lines() {
    for c in line.chars().rev() { 
        if c.is_numeric() {
            v2.push(c);
            break;
        }
    }
}

for it in v1.iter().zip(v2.iter_mut()) {
    let (c1, c2) = it;
    let mut s = String::from("");
    s.push(*c1);
    s.push(*c2);
    println!("{}",s);
    result += s.parse::<i32>().unwrap();
}

  result


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, 281);
    }
}