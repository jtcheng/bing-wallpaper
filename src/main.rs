use bing_wallpaper::{get_today_wallpaper, update_readme, update_wallpaper};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let wallpaper = match get_today_wallpaper() {
        Ok(wallpaper) => wallpaper,
        Err(err) => return Err(err),
    };

    update_readme(&wallpaper)?;
    update_wallpaper(&wallpaper)?;

    Ok(())
}
