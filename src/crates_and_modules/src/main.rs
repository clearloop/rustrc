extern crate crates_and_modules;

fn main() {
    println!("Hello in English: {}", crates_and_modules::english::greetings::hello());
    println!("Goodbye in English: {}", crates_and_modules::english::farewells::goodbye());

    println!("Hello in Japanese: {}", crates_and_modules::japanese::greetings::hello());
    println!("Goodbye in Japanese: {}", crates_and_modules::japanese::farewells::goodbye());
}
