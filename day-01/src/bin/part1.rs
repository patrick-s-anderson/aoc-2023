//use std::str::Chars;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!( "{}", output);
}

fn part1(input: &str) -> i32 {
    let mut v1 = vec![];
    let mut v2 = vec![];

    let mut result = 0;
    for line in input.to_string().lines() {
        for c in line.chars() { 
            if c.is_numeric() {
                v1.push(c);
                //result += c.to_digit(10).unwrap();
                break;
            }
        }
  }

  for line in input.to_string().lines() {
    for c in line.chars().rev() { 
        if c.is_numeric() {
            v2.push(c);
            //result += c.to_digit(10).unwrap();
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

// for c in v1.iter_mut() {
//     let mut s = String::from("");
//     s.push(c);
//   }
  result


    // .map(|line| {
    //     line.split_()
    //         .map(|col| col.parse::<i32>())
    //         .collect::<Result<Vec<_>, _>>()
    // })
    // .collect::<Result<Vec<_>, _>>()
    // .unwrap_or_else(|err| panic!("Cannot parse a column: {}", err))


    // .split("\n")
    // .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
    // .max()
    // .unwrap()
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