use std::collections::HashSet;

fn generatepasswords(length: usize, charset: &[char]) -> HashSet<String> {
    let mut passwords = HashSet::new();
    passwords_recursive(&mut passwords, length, charset, String::new());
    passwords
}

fn passwords_recursive(passwords: &mut HashSet<String>, length: usize, charset: &[char], current_password: String,){
    if current_password.len() == length {
        passwords.insert(current_password);
        return;
    }
    for &c in charset {
        let mut next_possible_password = current_password.clone();
        next_possible_password.push(c);
        passwords_recursive(passwords, length, charset, next_possible_password)
    }
}

fn main() {
    let charset = ['0','1','2','3','4', '5', '6'];
    let password_length = 10;

    let passwords = generatepasswords(password_length, &charset);
    let num_attempts = passwords.len();

    println!("Number of Attempts: {}", num_attempts);
}
