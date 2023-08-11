// mod games;
// mod hello;
mod dsa;

fn main() {
    // hello::print_hello();
    // games::guessing_game::play();

    let result = dsa::fib::run();
    println!("Result {}",result);
}
