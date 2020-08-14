use std::fs;
struct Node {}
fn main() {
    let raw: String = fs::read_to_string("test.txt").unwrap();
    for i in 0..raw.len() {
        let c = raw.chars().nth(i).unwrap();
        println!("{} {}", i, c);
        // loop {}
        if c == '{' {
            print!("{}", 1)
        }
        break;
    }
    // for i in tmp.chars() {
    //     println!("{}", i)
    // }
    // let lines: Vec<&str> = raw.split("\r\n").collect();
}