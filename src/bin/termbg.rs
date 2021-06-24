fn main() {
    match termbg::get_background_color() {
        Ok(c) => println!("{}", c),
        Err(e) => {
            eprintln!("error: {}", e);
            println!("unknown");
        }
    }
}
