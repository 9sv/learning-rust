use std::env;

fn is_even(num: i32) {
    if (num % 2) == 0 {
        println!("{0} is even", num);
    } else {
        println!("{0} is not even", num);
    }
}

fn main() {
    if !(env::args().len() > 1) {
        println!("gimme a number smh");
        std::process::exit(1);
    } else {
        let args: Vec<String> = env::args().collect();
        let num: i32 = match args[1].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("error parsing arg as i32");
                std::process::exit(1);
            },
        };
        is_even(num);
    }
}
