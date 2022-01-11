mod util;
use crate::util::actions;
use crate::util::wallpaper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallpaper = match wallpaper::get_wallpaper() {
        Ok(wallpaper) => wallpaper,
        Err(err) => return Err(err),
    };

    actions::update_readme(&wallpaper)?;
    actions::update_wallpaper(&wallpaper)?;

    Ok(())
}
