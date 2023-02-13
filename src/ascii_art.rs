use image::{DynamicImage, GenericImageView};

fn change_scope_to_ascii_greyscale(brightness: f64) -> char {
    let character = match brightness as u8 {
        0..=25 => '#',
        26..=50 => '&',
        51..=75 => 'o',
        76..=100 => ':',
        101..=125 => '*',
        126..=150 => '.',
        _ => ' ',
    };
    character
}

pub fn jpg_to_ascii_art(image: &DynamicImage) -> String {
    let mut ascii_art = String::new();

    for y in 0..image.height() {
        for x in 0..image.width() {
            let [r, g, b, _] = image.get_pixel(x, y).0;
            let brightness = (0.2126 * r as f64) + (0.7152 * g as f64) + (0.0722 * b as f64);
            let character = change_scope_to_ascii_greyscale(brightness);
            ascii_art.push(character);
        }
        ascii_art.push('\n');
    }
    ascii_art
}
