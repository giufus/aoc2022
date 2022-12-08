mod first;
mod second;

fn main() {
    println!("Hello, world!");
    first::run("src/inputs/first_input.json");
    second::run("src/inputs/second_input.json");
}
