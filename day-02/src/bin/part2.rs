use std::cmp;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    println!( "{}", output);
}

// what is the sum of the game IDs that can have 12 red cubes, 13 green cubes, and 14 blue cubes?
fn part2(input: &str) -> i32 {
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

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for n in 1..v.len()-1 {
            if 0 == n%2 {
                continue
            }
            if v[n+1] == "red" {
                red = cmp::max(red, v[n].parse::<i32>().unwrap());
            }
            if v[n+1] == "green" {
                green = cmp::max(green, v[n].parse::<i32>().unwrap());
            }
            if v[n+1] == "blue" {
                blue = cmp::max(blue, v[n].parse::<i32>().unwrap());
            }
        }
        f += red*blue*green;
    }
    //println!("{}", f);
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 2286);
    }
}