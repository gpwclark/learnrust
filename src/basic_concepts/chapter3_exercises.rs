pub fn change(s: &mut String) {
                            s.push_str(", world!")
                                                   }

pub fn fib(x: u32) -> f64 {
                        fib_recur(x, 1.0, 0.0)
                                                 }

pub fn fib_recur(mut count: u32, prev: f64, curr: f64) -> f64 {
    if count == 0 {
        curr
    } else {
        count -= 1;
        fib_recur(count, curr, prev + curr)
    }
}

pub fn f_to_c(deg: f64) -> f64 {
                             ((deg - 32.0) * 5.0) / 9.0
                                                       }

pub fn c_to_f(deg: f64) -> f64 {
    (deg / 5.0) * 9.0 + 32.0
}
