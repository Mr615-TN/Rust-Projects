fn main() {
    let new_variable = 9;
    println!("This prints {}", new_variable);
    {
        let new_variable = "Hello There!";
        println!("This prints {}", new_variable);
    }
    println!("This prints the original variable which is {}", new_variable);
}