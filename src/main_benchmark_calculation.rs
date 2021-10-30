use std::fs::File;
use std::io::{stdin, BufRead, Read};
use std::time::Instant;

struct Testcase {
    dog_portions_left: u64,
    cat_portions_left: u64,
    cat_food_increment: u64,
    food_chain: String,
}

fn main() {
    let mut input = String::new();
    File::open(r"C:\Users\Ch4s3r\Downloads\test_data\test_set_2\ts2_input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let iterations = 10;
    let mut list: Vec<&str> = Vec::with_capacity(iterations * 100);

    dbg!("starting");
    let sum: u128 = (0..iterations)
        .map(|_| {
            let mut lines = input.lines();
            let number_of_testcases = lines.next().unwrap().parse::<u64>().unwrap();

            let testcases = (1..=number_of_testcases)
                .map(|_| {
                    let numbers = lines.next().unwrap();
                    let numbers = numbers
                        .split(' ')
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    let food_chain = lines.next().unwrap();
                    Testcase {
                        dog_portions_left: numbers[1],
                        cat_portions_left: numbers[2],
                        cat_food_increment: numbers[3],
                        food_chain: food_chain.to_string(),
                    }
                })
                .collect::<Vec<_>>();

            let stopwatch = Instant::now();
            for testcase in testcases {
                let result = calculate_testcase(
                    testcase.dog_portions_left,
                    testcase.cat_portions_left,
                    testcase.cat_food_increment,
                    &testcase.food_chain,
                );
                list.push(result);
                // println!("Case #{}: {}", i, result);
            }
            stopwatch.elapsed().as_nanos()
        })
        .into_iter()
        .sum();
    dbg!(list.len(), sum / iterations as u128);
}

fn calculate_testcase(
    dog_portions_left: u64,
    cat_portions_left: u64,
    cat_food_increment: u64,
    food_chain: &str,
) -> &'static str {
    let mut dog_portions_left = dog_portions_left;
    let mut cat_portions_left = cat_portions_left;
    let mut animal_starving = false;
    let mut result = "YES";
    for animal in food_chain.chars() {
        if !animal_starving {
            match animal {
                'C' => {
                    if cat_portions_left > 0 {
                        cat_portions_left -= 1;
                    } else {
                        animal_starving = true;
                    }
                }
                _ => {
                    if dog_portions_left > 0 {
                        dog_portions_left -= 1;
                        cat_portions_left += cat_food_increment;
                    } else {
                        result = "NO";
                        break;
                    }
                }
            }
        } else if animal == 'D' {
            result = "NO";
            break;
        }
    }
    result
}

// fn next_line() -> String {
//     stdin().lock().lines().next().unwrap().unwrap()
// }
