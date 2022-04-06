fn largest(l: &[i32]) -> i32 {
    let mut _largest = l[0];
    for &num in l {
        if num > _largest {
            _largest = num;
        }
    }
    _largest
}

fn main() {
    let nums = vec![25, 64, 323, 43, 1124, 65543, 69, 62];
    println!("The largest number in that list is {}", largest(&nums));
}
