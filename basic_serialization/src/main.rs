use serde::{Deserialize, Serialize};
use serde_json::to_string;
use serde_json::Error;

#[derive(Serialize, Deserialize)]
struct Pet {
    animal_type: String,
    animal_name: String,
    year_born: i32,
}

fn main() {
    let pet_1: Pet = Pet{animal_type: "Snake".to_string(), animal_name: "Shelby".to_string(), year_born: 2020};
    
    let pet_ser: Result<String, Error> = to_string(&pet_1);

    if pet_ser.is_ok() {
        println!("{}", pet_ser.ok().unwrap());
    }
    else {
        println!("{:#?}", pet_ser.err());
    }
}
