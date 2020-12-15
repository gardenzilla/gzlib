use gzlib::id::*;

fn main() {
    (150_000..151_000).into_iter().for_each(|i| {
        let id = generate_id(i, IdKind::LuhnTwo);
        println!(
            "{} - {} # {:?}",
            id,
            id.to_hex(),
            id.to_hex().to_luhn_object()
        );
    });
}
