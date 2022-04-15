fn main() {
    let mut n = 0;

    loop {
        println!("{}", fib(n));
        n += 1;
    }
}

fn fib(n: i128) -> i128{
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}