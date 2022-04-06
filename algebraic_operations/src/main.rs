use std::env;

fn main() {
    match env::args().len() > 3 {
        true => {
            println!("Im only smart enough to handle 2 args");
            std::process::exit(1);
        },
        false => {
            let args: Vec<String> = env::args().collect();
            let arg1: i64 = args[1].trim().parse().expect("sys arg 1 not int parseable");
            let arg2: i64 = args[2].trim().parse().expect("sys arg 2 not int parseable");

            println!("{} + {} = {}", arg1, arg2, arg1 + arg2);
            println!("{} - {} = {}", arg1, arg2, arg1 - arg2);
            println!("{} x {} = {}", arg1, arg2, arg1 * arg2);
            println!("{} / {} = {}", arg1, arg2, arg1 as f64 / arg2 as f64);
            println!("{} % {} = {} (Modulo)", arg1, arg2, arg1 % arg2);
            println!("{} > {} = {}", arg1, arg2, arg1 > arg2);
            println!("{} < {} = {}", arg1, arg2, arg1 < arg2);
            println!("{} == {} = {}", arg1, arg2, arg1 == arg2);
        }
    }
}
