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

/*
Using let makes the variable immutable/static.
using the prefix mut after the variable let allows one to change the value of a variable while still using the orignal name

for example the following code is no bueno because the keyword mut is not there:
    let new_number = 9;
    new_number = 10;

to fix it, you use the keyword mut and it makes the variable new_number not static/muttable:
    let mut new_number = 9;
    new_number = 10;
Shadowing a variable is basically blocking a variable so that you can change the variable a bunch of times
*/