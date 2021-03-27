use std::cmp::Ordering;

pub struct Guess {
    val: i32,
}

impl Guess {
    pub fn new (val: i32) -> Guess {
        if val < 1 || val > 100 {
            panic!("Guess must be non neg int 100 or less. got: {}.", val);
        }

        Guess { val }
    }

    pub fn val(&self) -> i32 {
        self.val
    }
}

pub fn game(secret_number: i32) {
    println!("Guess the number!");
    println!("Please input your guess: ");

    loop {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let g = Guess::new(guess);
        let g_val = g.val();
        println!("You guessed: {}", g_val);

        let i_guessed_it = secret_number;

        println!("Secret #: {}.", secret_number);
        match secret_number.cmp(&i_guessed_it) {
        //match g_val.cmp(&i_guessed_it) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
        }
    }
}
