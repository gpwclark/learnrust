mod restaurant;

pub use crate::restaurant::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist("meowman");
    hosting::add_to_waitlist("meosmasherson");
    hosting::add_to_waitlist("meowane");
}
