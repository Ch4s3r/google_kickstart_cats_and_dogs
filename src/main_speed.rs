use std::io::{BufRead, stdin};

fn main() {
    let number_of_testcases = stdin().lock().lines().next().unwrap().unwrap().parse::<u64>().unwrap();
    for i in 1..=number_of_testcases {
        let numbers = stdin().lock().lines().next().unwrap().unwrap()
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let mut dog_portions_left = numbers[1];
        let mut cat_portions_left = numbers[2];
        let cat_food_increment = numbers[3];
        let food_chain = stdin().lock().lines().next().unwrap().unwrap();
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
            } else {
                if animal == 'D' {
                    result = "NO";
                    break;
                }
            }
        }
        println!("Case #{}: {}", i, result);
    }
}