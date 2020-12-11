use std::env;

use gzlib::id;

enum Action {
    Create,
    CreateHex,
    Validate,
    ValidateHex,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (action, id) = match args.len() {
        1 | 2 => panic!(
            "Hiba! 2 paraméter szükséges! id --create|--create_hex|--validate|--validate_hex ID"
        ),
        3 => (
            match args[1].as_str() {
                "--create" => Action::Create,
                "--create_hex" => Action::CreateHex,
                "--validate" => Action::Validate,
                "--validate_hex" => Action::ValidateHex,
                _ => panic!("Hiba! Nem megfelelő action! Vagy create, vagy validate!"),
            },
            &args[2],
        ),
        _ => panic!("Hiba! Túl sok paraméter!"),
    };

    let id = match action {
        Action::ValidateHex => u64::from_str_radix(&args[2], 16)
            .expect("A megadott ID nem HEX, vagy nem pozitív egész szám"),
        _ => args[2]
            .parse::<u64>()
            .expect("A megadott ID nem pozitív egész szám!"),
    };

    match action {
        Action::Create => println!("{}", id::luhn::make_id(id)),
        Action::CreateHex => println!("{:x}", id::luhn::make_id(id)),
        Action::Validate => println!("{}", id::luhn::is_valid(id)),
        Action::ValidateHex => println!("{}", id::luhn::is_valid(id)),
    }
}
