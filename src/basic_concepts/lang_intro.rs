pub fn run() {
    let x = f_to_c(72.0);
    println!("72 in celsius: {}", x);
    let x = c_to_f(90.0);
    println!("90 in fah: {}", x);
    let f = 222;
    let x = fib(f);
    println!("fib of {} in fah: {}", f, x);
    let a = ["deck thehalls", "with bells " , " of holy", "fa lala"];
    for elem in a.iter() {
        println!("elem! {}", elem);
    }
}

fn fib(x: u32) -> f64 {
    fib_recur(x, 1.0, 0.0)
}

fn fib_recur(mut count: u32, prev: f64, curr: f64) -> f64 {
    if count == 0 {
        curr
    } else {
        count -= 1;
        fib_recur(count, curr, prev + curr)
    }
}

fn f_to_c(deg: f64) -> f64 {
    ((deg - 32.0) * 5.0) / 9.0
}

fn c_to_f(deg: f64) -> f64 {
    (deg / 5.0) * 9.0 + 32.0
}
