fn main() {
    let nums: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in nums {
        println!("{0} ** 2 = {1}", i, i.pow(2));
    }
}
