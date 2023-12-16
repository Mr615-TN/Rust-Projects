fn squared(number: i32) -> i32 {
    number * number
}

fn main() {
    let new_variable = {
        let y = 21;
        let x = 22;
        let x = squared(x);
        let x = x + y;
        x
    };
    println!("The number I got after inputting x as 22 and y as 21 is {}", new_variable)
}