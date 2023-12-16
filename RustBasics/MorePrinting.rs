fn main() {
    println!("Here are two escape characters: \\n and \\t");
}

/*
Rust allows one to print in a multitude of ways. 

One way can be like the following code:
fn main() {
    print!("\t You can print this on one line with a tab \n and this one another line")
}

Another thing one can do is print over many lines. The following code will show that.
fn main() {
    println!("Inside quotes
    you can write over
    many lines
    and it will print just fine.");
    
    println!("If you forget to write
    on the left side, the spaces
    will be added when you print.");
}

In rust, there are two escape characters adn they are \n and \t. To print them and show them in a string we would do the following:
fn main() {
    println!("Here are two escape characters: \\n and \\t");
}
*/