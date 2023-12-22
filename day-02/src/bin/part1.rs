fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!( "{}", output);
}

// what is the sum of the game IDs that can have 12 red cubes, 13 green cubes, and 14 blue cubes?
fn part1(input: &str) -> i32 {
    let mut f = 0;
    for line in input.to_string().lines() {
        let mut result = str::replace(line, ":", "");
        result = str::replace(&result, "Game ", "");
        result = str::replace(&result, ",", "");
        result = str::replace(&result, ";", "");
        //result = str::replace(&result, " ", "");
        //let id = result.chars().next().unwrap();
        let v: Vec<&str> = result.split(' ').collect();
        //let id = v.clone().into_iter().nth(0).unwrap().parse::<i32>().unwrap();
       // println!("{:?}", v);
        //println!( "{}", id);

        let mut red = 1;
        let mut blue = 1;
        let mut green = 1;

        for n in 1..v.len()-1 {
            if 0 == n%2 {
                continue
            }
            if (v[n].parse::<i32>().unwrap() > 12) & (v[n+1] == "red") {
                red = 0;
            }
            if (v[n].parse::<i32>().unwrap() > 13) & (v[n+1] == "green") {
                green = 0;
            }
            if (v[n].parse::<i32>().unwrap() > 14) & (v[n+1] == "blue") {
                blue = 0;
            }
        }
        if (red == 1) & (blue == 1) & (green == 1) {
            f += v[0].parse::<i32>().unwrap();
            println!("{:?}", v);
        }
    }
    //println!("{}", f);
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 8);
    }
}