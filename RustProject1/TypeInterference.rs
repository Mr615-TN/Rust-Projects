fn multiply(number_one: i32, number_two: i32) {
    let new_numero = number_one * number_two;
    println!("{} multiplied with {} is {}", number_one, number_two, new_numero)
}

fn main() {
    multiply(21, 22);
    let other_num = 69;
    let other_other_num = 420;
    multiply(other_num, other_other_num);
}
