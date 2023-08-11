use std::io;

pub fn run () -> i32 {
  println!("Please enter the number you would like to calculate the fibonacci sequence for");
  
  let mut user_input = String::new();
  io::stdin().read_line(&mut user_input).expect("Failed to read string");
  let n: i32 = user_input.trim().parse().expect("Please type a number!");

  return fib_n(n);
}

fn fib_n(num:i32) -> i32 {
  match num {
    0 => 0,
    1 => 1,
    _ => fib_n(num-1) + fib_n(num-2)
  }
}
