pub fn borrow_stuff() {
    //TODO pepper with more examples from chp4
    let mut mutable_string = String::from("hello");
    // mutable reference, max of 1 per value.
    change(&mut mutable_string);
    println!("the old familiar: {}.", mutable_string);

    let string = String::from("hello");
    // change sig has mutable borrow but string is declared immutable
    //f::change(&mut string);
    // println!("the old familiar: {}.", string);

    // value of string moves into other_string and is dropped.
    let other_string = string;
    // println! fails because value was already moved, println can't borrow.
    //println!("STRING: {}.", string)

} // all values defined go out of scope and are dropped.

fn change(s: &mut String) {
    s.push_str(", world!")
}
