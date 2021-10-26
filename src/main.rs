use std::error::Error;
use std::fmt::Debug;
use std::io::{BufRead, Read, stdin, stdout, Write};

fn main() {
    eval(&mut stdin().lock(), &mut stdout().lock())
}

fn get_input(input: &mut (impl Read + BufRead)) -> String {
    let mut buffer = String::new();
    input.read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}

#[derive(Debug)]
struct Food {
    dog_portions_left: u64,
    cat_portions_left: u64,
    cat_food_increment: u64,
}

trait Animal: Debug {
    fn eat(&self, food: &mut Food) -> Result<(), Box<dyn Error>>;
    fn starving_causes_loss(&self) -> bool;
}

#[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn eat(&self, food: &mut Food) -> Result<(), Box<dyn Error>> {
        if food.cat_portions_left > 0 {
            food.cat_portions_left -= 1;
            dbg!(&food);
            Ok(())
        } else {
            Err("No cat food left.".into())
        }
    }

    fn starving_causes_loss(&self) -> bool {
        false
    }
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn eat(&self, food: &mut Food) -> Result<(), Box<dyn Error>> {
        if food.dog_portions_left > 0 {
            food.dog_portions_left -= 1;
            food.cat_portions_left += food.cat_food_increment;
            dbg!(&food);
            Ok(())
        } else {
            Err("No dog food left.".into())
        }
    }

    fn starving_causes_loss(&self) -> bool {
        true
    }
}

fn eval(input: &mut (impl Read + BufRead), output: &mut impl Write) {
    let number_of_tests = get_input(input).parse::<u64>().unwrap();
    for i in 1..number_of_tests + 1 {
        let numbers = get_input(input)
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let _total_animals = numbers[0];
        let mut food = Food {
            dog_portions_left: numbers[1],
            cat_portions_left: numbers[2],
            cat_food_increment: numbers[3],
        };
        let food_chain = get_input(input);
        dbg!(&food_chain);
        let animals = food_chain
            .chars()
            .map(|animal_char| -> Box<dyn Animal> {
                if animal_char == 'D' {
                    Box::new(Dog)
                } else {
                    Box::new(Cat)
                }
            }).collect::<Vec<_>>();

        let mut first_starving_animal = animals.len();
        for (index, animal) in animals.iter().enumerate() {
            dbg!(&animal);
            if let Err(_) = animal.eat(&mut food) {
                first_starving_animal = index;
                break;
            }
        }

        let mut result = "YES".to_string();
        for animal in &animals[first_starving_animal..] {
            if animal.starving_causes_loss() {
                result = "NO".to_string();
                break;
            }
        }

        output.write(format!("Case #{}: {}\n", i, result).as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::*;

    #[test]
    fn it_works() {
        let mut input = "3
            6 10 4 0
            CCDCDD
            4 1 2 0
            CCCC
            4 2 1 0
            DCCD
        "
            .as_bytes();
        let mut output: Vec<u8> = Vec::new();
        eval(&mut input, &mut output);
        assert_eq!(
            r"Case #1: YES
Case #2: YES
Case #3: NO
",
            from_utf8(output.as_slice()).unwrap()
        )
    }

    #[test]
    fn it_works2() {
        let mut input = "2
12 4 2 2
CDCCCDCCDCDC
8 2 1 3
DCCCCCDC
        "
            .as_bytes();
        let mut output: Vec<u8> = Vec::new();
        eval(&mut input, &mut output);
        assert_eq!(
            r"Case #1: YES
Case #2: NO
",
            from_utf8(output.as_slice()).unwrap()
        )
    }
}
