fn main() {
    let my_number = 21;
    let reference1 = &my_number;
    let reference2 = &reference1;
    let reference3 = &reference2;
    let reference4 = &reference3;
    let reference5 = &&&&&my_number;
}