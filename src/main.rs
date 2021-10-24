#[macro_use]
extern crate indoc;

use std::error::Error;
use std::fmt::Debug;
use std::io;
use std::io::{BufRead, Read, stdin, stdout, Write};

use indoc::indoc;

fn main() {
    eval(&mut stdin().lock(), &mut stdout().lock())
}

fn get_input(
    input: &mut (impl Read + BufRead)
) -> String {
    let mut buffer = String::new();
    input.read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}

struct Food {
    dog: i32,
    cat: i32,
    cat_food_increment: i32,
}

trait Animal: Debug {
    fn eat(&self, food: &mut Food) -> Result<(), Box<dyn Error>>;
    fn starving_causes_lose(&self) -> bool;
}

#[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn eat(&self, food: &mut Food) -> Result<(), Box<dyn Error>> {
        if food.cat > 0 {
            food.cat -= 1;
            Ok(())
        } else {
            Err("No cat food left.".into())
        }
    }

    fn starving_causes_lose(&self) -> bool {
        false
    }
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn eat(&self, food: &mut Food) -> Result<(), Box<dyn Error>> {
        if food.dog > 0 {
            food.dog -= 1;
            food.cat += food.cat_food_increment;
            Ok(())
        } else {
            Err("No dog food left.".into())
        }
    }

    fn starving_causes_lose(&self) -> bool {
        true
    }
}

fn eval(
    input: &mut (impl Read + BufRead),
    output: &mut impl Write,
) {
    let number_of_tests = get_input(input).parse::<i32>().unwrap();
    for i in 1..number_of_tests + 1 {
        let numbers = get_input(input).split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let total_animals = numbers[0];
        let mut food = Food {
            dog: numbers[1],
            cat: numbers[2],
            cat_food_increment: numbers[3],
        };
        let food_chain = get_input(input);
        let mut result = "YES".to_string();
        let dogs_not_fed = food_chain
            .chars()
            .map(|animal_char| -> Box<dyn Animal> { if animal_char == 'D' { Box::new(Dog) } else { Box::new(Cat) } })
            .map(|animal| {
                match animal.eat(&mut food) {
                    Ok(_) => { Ok(()) }
                    Err(e) => { Err(animal) }
                }
            })
            .filter(|result| {
                match result {
                    // todo check how to return true only if the animal is a Dog inside the Box in the Error
                    // Err(Box(animal, ..)) =>,
                    _ => { false }
                }
            }).count();

        output.write(format!("Case #{}: {}\n", i, result).as_bytes());
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
        ".as_bytes();
        let mut output: Vec<u8> = Vec::new();
        eval(&mut input, &mut output);
        assert_eq!(
            r"Case #1: YES
Case #2: YES
Case #3: NO
"
            , from_utf8(output.as_slice()).unwrap()
        )
    }
}