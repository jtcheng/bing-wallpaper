use super::wallpaper::Wallpaper;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{prelude::*, BufReader, BufWriter};

fn read_file_contents(fname: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(fname)?;
    let contents: Vec<_> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.trim().is_empty())
        .collect();

    Ok(contents)
}

pub fn update_readme(w: &Wallpaper) -> Result<(), Box<dyn Error>> {
    let readme = "README.md";
    let temp = "TEMP.md";
    let contents = match read_file_contents(readme) {
        Ok(contents) => contents,
        Err(err) => return Err(err),
    };

    let mut lines: Vec<_> = Vec::with_capacity(13);
    lines.push("## Bing Wallpaper".to_string());
    lines.push(w.to_large());
    lines.push("|        |        |        |".to_string());
    lines.push("| :----: | :----: | :----: |".to_string());
    if contents.is_empty() {
        lines.push(w.to_small());
    } else {
        for line in contents[4..].iter() {
            for w in line.split('|').filter(|&x| !x.is_empty()) {
                lines.push(w.to_string());
            }
        }
        if lines.len() == lines.capacity() {
            lines.pop();
        }
        lines.insert(4, w.to_small());
    }

    {
        let file = fs::File::create(temp)?;
        let mut buf = BufWriter::new(file);
        for line in lines[..=3].iter() {
            buf.write_fmt(format_args!("{}\n", line))?;
        }
        for (index, line) in lines[4..].iter().enumerate() {
            if index % 3 == 0 {
                buf.write_all(b"|")?;
            }
            buf.write_fmt(format_args!("{}|", line))?;
            if index % 3 == 2 {
                buf.write_all(b"\n")?;
            }
        }
    }

    fs::rename(temp, readme)?;

    Ok(())
}

pub fn update_wallpaper(w: &Wallpaper) -> Result<(), Box<dyn Error>> {
    let wallpaper = w.get_year() + "-wallpaper.md";
    let temp = "temp.md";
    let contents = match read_file_contents(&wallpaper) {
        Ok(contents) => contents,
        Err(err) => return Err(err),
    };

    {
        let file = fs::File::create(temp)?;
        let mut buf = BufWriter::new(file);
        buf.write_fmt(format_args!("{}\n", "## Bing Wallpaper"))?;
        buf.write_fmt(format_args!("{}\n", w.to_markdown()))?;
        if !contents.is_empty() {
            for line in contents[1..].iter() {
                buf.write_fmt(format_args!("{}\n", line))?;
            }
        }
    }

    fs::rename(temp, wallpaper)?;

    Ok(())
}
