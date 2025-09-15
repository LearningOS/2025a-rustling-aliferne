// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(
        // TODO:
    );

    string_uppercase(
        // TODO:
    );
}

// Should not take ownership
fn get_char(data: String) -> char {
    // TODO:
}

// Should take ownership
fn string_uppercase(mut data: String) {
    // TODO:
}
