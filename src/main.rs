#[macro_use]
extern crate enum_display_derive;

mod basic_concepts;
use basic_concepts as bc;
use bc::lang_intro;
use bc::borrowing as b;
use bc::enum_and_match as em;
pub mod advent1;

fn main() {
    lang_intro::run();

    // each value in rust has a variable that's called its owner.
    // there can be only one at a time.
    // when the owner goes out of scope the value will be dropped.
    b::borrow_stuff();

    let my_string = match em::enums() {
        Some(str) => {
            str
        }
        None => {
            String::from("")
        }
    };
    println!("From enums: {}.", my_string);

    //bc::guess::game();

    use_lib();
    advent1::print_nums();
}

fn use_lib() {
    learn::eat_at_restaurant();
    learn::hosting::add_to_waitlist("myself");
}
