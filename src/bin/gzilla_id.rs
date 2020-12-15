use std::env;

use gzlib::id::*;

enum Action {
    Create,
    Check,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (action, id) = match args.len() {
        1 | 2 => panic!(
            "Hiba! 2 paraméter szükséges! id --create|--create_hex|--validate|--validate_hex ID"
        ),
        3 => (
            match args[1].as_str() {
                "create" => Action::Create,
                "check" => Action::Check,
                _ => panic!("Hiba! Nem megfelelő action! Vagy create, vagy validate!"),
            },
            &args[2],
        ),
        _ => panic!("Hiba! Túl sok paraméter!"),
    };

    match action {
        Action::Create => println!(
            "{}",
            generate_id(
                id.parse::<u64>()
                    .expect("A megadott ID nem pozitív egész szám"),
                IdKind::LuhnTwo
            )
            .to_hex()
        ),
        Action::Check => println!("{}", id.luhn_check().is_ok()),
    }
}
