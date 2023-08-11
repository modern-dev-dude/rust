use rand::Rng;
use std::io;

pub fn play() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let mut guess_as_num: i32 = convert_string_input_to_num(guess);

    let generated_num: i32 = random_num_generator((1, 10));

    if guess_as_num == generated_num {
        println!("You guessed correct!");
    }else {
        println!("You guessed incorrect!");
    }

    while guess_as_num != generated_num {
        println!("Please guess again");
        guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line!");
        guess_as_num = convert_string_input_to_num(guess);
        if guess_as_num == generated_num {
            println!("You guessed correct!");
        }else {
            println!("You guessed incorrect!");
        }
    }
}

fn random_num_generator(range:(i32, i32)) -> i32{
    let mut rng = rand::thread_rng();
    let random_num: i32 =rng.gen_range(range.0..range.1);
    return random_num;
}

fn convert_string_input_to_num(input: String) -> i32 {
    return input.trim().parse().expect("Please type a number!");
}

