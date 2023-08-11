mod games;
mod hello;

fn main() {
    hello::print_hello();
    
    games::guessing_game::play();
}
