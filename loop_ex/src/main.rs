use std::time::Instant;
use std::io;

fn main() {
    let mut x = 0;
    let now = Instant::now();

    while x < 69421 {
        println!("{0}", x);
        x += 1;
    }
    println!("counted to {} in {}.{} seconds", x - 1, now.elapsed().as_secs(), now.elapsed().as_millis());

    let mut h = String::new();
    io::stdin()
        .read_line(&mut h)
        .expect("how tf you break input reading nigga"); // read input to avoid program exiting on end
}
