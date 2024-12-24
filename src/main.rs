fn main() {
    // Write a short program that prints each number from 1 to 15 on a new line.
    // For each multiple of 3, print "FIZZ".
    // For each multiple of 5, print "BUZZ".
    // For numbers which are multiples of both 3 and 5, print "FIZZBUZZ".
    // If no of these statements are satisfied print out the number.

    let n: u32 =150;

    for i in 1..n + 1 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FIZZBUZZ");
        } else if i % 3 == 0 {
            println!("FIZZ");
        } else if i % 5 == 0 {
            println!("BUZZ");
        } else {
            println!("{}", i);
        }
    }
}
