mod ascii_art;
use anyhow::Result;

fn main() -> Result<()> {
    let image = image::open("assets/image.jpg")?;
    println!("{}", ascii_art::jpg_to_ascii_art(&image));
    Ok(())
}