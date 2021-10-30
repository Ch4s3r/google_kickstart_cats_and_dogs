use std::fs::File;
use std::io::{stdin, BufRead, Read};
use std::time::Instant;

fn main() {
    let mut input = String::new();
    File::open(r"C:\Users\Ch4s3r\Downloads\test_data\test_set_2\ts2_input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let stopwatch = Instant::now();
    let mut lines = input.lines();
    let number_of_testcases = lines.next().unwrap().parse::<u64>().unwrap();
    for i in 1..=number_of_testcases {
        let numbers = lines.next().unwrap();
        let food_chain = lines.next().unwrap();
        let _result = calculate_testcase(numbers, food_chain);
        // println!("Case #{}: {}", i, result);
    }
    dbg!(stopwatch.elapsed());
}

fn calculate_testcase(numbers: &str, food_chain: &str) -> &'static str {
    let numbers = numbers
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut dog_portions_left = numbers[1];
    let mut cat_portions_left = numbers[2];
    let cat_food_increment = numbers[3];
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
