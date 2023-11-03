fn main() {
    // This is a comment
    /* This is also a comment */
    // Both ways have their applications. 
        // For example, the first one is good for single line comments and the second one is good for multiline comments
    println!("hello world!");
    println!("Printing my first number which is {}", 8);
    let random_number = /* You can use this comment in the middle of code as well */ 100;
    println!("{}",random_number);
    println!("{}",random_number as u8 as char);
    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
}