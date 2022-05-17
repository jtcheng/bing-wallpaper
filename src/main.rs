mod util;
use crate::util::actions;
use crate::util::wallpaper;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let wallpaper = wallpaper::get_wallpaper()?;

    actions::update_readme(&wallpaper)?;
    actions::update_wallpaper(&wallpaper)?;

    Ok(())
}
