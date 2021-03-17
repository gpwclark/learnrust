mod basic_concepts;

fn main() {
    use basic_concepts::chapter3_exercises as f;
    let x = f::f_to_c(72.0);
    println!("72 in celsius: {}", x);
    let x = f::c_to_f(72.0);
    println!("72 in fah: {}", x);
    let f = 222;
    let x = f::fib(f);
    println!("fib of {} in fah: {}", f, x);
    let a = ["deck thehalls", "with bells " , " of holy", "fa lala"];
    for elem in a.iter() {
        println!("elem! {}", elem);
    }

    let mut s = String::from("hello");
    f::change(&mut s);
    println!("the old familiar: {}.", s);

    learn::eat_at_restaurant();
    learn::hosting::add_to_waitlist("myself");

    use basic_concepts::guess;
    guess::game();
}
