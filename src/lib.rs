use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{prelude::*, BufReader};

pub struct Wallpaper {
    url: String,
    date: String,
    copyright: String,
}

impl Wallpaper {
    fn to_markdown(&self) -> String {
        format!("{} | [{}]({})  ", self.date, self.copyright, self.url)
    }

    fn to_large(&self) -> String {
        let large_url = self.url.clone() + "&w=1024";
        format!(
            "![]({})Today: [{}]({})",
            large_url, self.copyright, self.url
        )
    }

    fn to_small(&self) -> String {
        let small_url = self.url.clone() + "&pid=hp&w=384&h=216&rs=1&c=4";
        format!(
            "![]({}){} [download 4k]({})",
            small_url, self.date, self.url
        )
    }

    fn get_year(&self) -> String {
        self.date[..4].to_string()
    }
}

pub fn get_today_wallpaper() -> Result<Wallpaper, Box<dyn Error>> {
    const BING_API: &str = "https://cn.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&nc=1615276059126&pid=hp&FORM=BEHPTB&uhd=1&uhdwidth=3840&uhdheight=2160";
    const BING_URL: &str = "https://cn.bing.com";

    let resp = reqwest::blocking::get(BING_API)?.json::<serde_json::Value>()?;
    let json = &resp["images"][0];

    Ok(Wallpaper {
        url: BING_URL.to_string() + json["url"].as_str().unwrap().splitn(2, '&').next().unwrap(),
        date: json["enddate"].as_str().unwrap().to_string(),
        copyright: json["copyright"].as_str().unwrap().to_string(),
    })
}

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
        for l in contents[4..].iter() {
            for s in l.split('|').filter(|&x| !x.is_empty()) {
                lines.push(s.to_string());
            }
        }
        if lines.len() == 13 {
            lines.pop();
        }
        lines.insert(4, w.to_small());
    }

    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(temp)?;
        for l in lines[..=3].iter() {
            file.write_fmt(format_args!("{}\n", l))?;
        }
        for (pos, l) in lines[4..].iter().enumerate() {
            if pos % 3 == 0 {
                file.write_all("|".as_bytes())?;
            }
            file.write_fmt(format_args!("{}|", l))?;
            if pos % 3 == 2 {
                file.write_all("\n".as_bytes())?;
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
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(temp)?;
        file.write_all("## Bing Wallpaper\n".as_bytes())?;
        file.write_all(w.to_markdown().as_bytes())?;
        file.write_all("\n".as_bytes())?;
        if !contents.is_empty() {
            for l in contents[1..].iter() {
                file.write_fmt(format_args!("{}\n", l))?;
            }
        }
    }

    fs::rename(temp, wallpaper)?;

    Ok(())
}
