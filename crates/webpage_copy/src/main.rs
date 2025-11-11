use std::error::Error;
use std::io;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://store-site-backend-static-ipv4.ak.epicgames.com/freeGamesPromotions";
    let body = reqwest::blocking::get(url)?.text()?;

    let _ = write_file(body);

    Ok(())
}

fn write_file(contents: String) -> io::Result<()> {
    let file = File::create("page.json")?;
    let mut writer = io::BufWriter::new(file);
    writer.write_all(contents.as_bytes())?;

    Ok(())
}
