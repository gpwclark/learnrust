extern crate rand;
#[macro_use]
extern crate enum_display_derive;
extern crate regex;

//use serde::{Serialize, Deserialize};

use rand::Rng;

mod basic_concepts;

use basic_concepts as bc;
use bc::lang_intro;
use bc::borrowing as b;
use bc::enum_and_match as em;

mod data_structures;
mod advent_of_code;

//#[derive(Serialize, Deserialize)]
//pub struct MyConfig {
//    pub day1_magic_num: u32,
//}
//
///// `MyConfig` implements `Default`
//impl ::std::default::Default for MyConfig {
//    fn default() -> Self { Self { day1_magic_num: 2020 } }
//}

fn main() {
//    let cfg = confy::load("learn").expect("Config load failure.");
//    let num_to_sum_to: u32 = cfg.day1_magic_num;
    let num_to_sum_to: u32 = 2020;
//
//    println!("config: {}.", cfg);
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

    let secret_number = rand::thread_rng().gen_range(1, 101);
    bc::guess::game(secret_number);

    data_structures::do_a_struct();

    use_lib();
    advent_of_code::advent1::solve_puzzle_1(num_to_sum_to);
    advent_of_code::advent1::solve_puzzle_2(num_to_sum_to);

    data_structures::generics();

  //  let dangling_reference = dangle();
}

//Rules of References
//- there can be N immutable references XOR 1 immutable reference.
//- references must always be valid
// fn dangle() -> &mut String {
//    let mut my_string = String::from("string");
//    &mut my_string
//}

fn use_lib() {
    learn::eat_at_restaurant();
    learn::hosting::add_to_waitlist("myself");
}
