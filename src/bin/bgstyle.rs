use termbg::*;

fn main() {
    let r = match termbg::get_background_color() {
        Ok(c) => BackgroundStyle::from(c),
        Err(_) => BackgroundStyle::Unknown,
    };

    println!("{}", r);
}
