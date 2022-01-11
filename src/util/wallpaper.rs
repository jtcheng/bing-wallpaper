pub struct Wallpaper {
    url: String,
    date: String,
    copyright: String,
}

impl Wallpaper {
    const fn new(url: String, date: String, copyright: String) -> Self {
        Wallpaper {
            url,
            date,
            copyright,
        }
    }

    pub fn to_markdown(&self) -> String {
        format!("{} | [{}]({})  ", self.date, self.copyright, self.url)
    }

    pub fn to_large(&self) -> String {
        let large_url = self.url.clone() + "&w=1024";
        format!(
            "![]({})Today: [{}]({})",
            large_url, self.copyright, self.url
        )
    }

    pub fn to_small(&self) -> String {
        let small_url = self.url.clone() + "&pid=hp&w=384&h=216&rs=1&c=4";
        format!(
            "![]({}){} [download 4k]({})",
            small_url, self.date, self.url
        )
    }

    pub fn get_year(&self) -> String {
        self.date[..4].to_string()
    }
}

pub fn get_wallpaper() -> Result<Wallpaper, Box<dyn std::error::Error>> {
    const BING_API: &str = "https://cn.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&nc=1615276059126&pid=hp&FORM=BEHPTB&uhd=1&uhdwidth=3840&uhdheight=2160";
    const BING_URL: &str = "https://cn.bing.com";

    let resp = reqwest::blocking::get(BING_API)?.json::<serde_json::Value>()?;
    let json = &resp["images"][0];
    let url = json["url"].as_str().unwrap();

    Ok(Wallpaper::new(
        BING_URL.to_string() + url.split_once('&').map_or(url, |x| x.0),
        json["enddate"].as_str().unwrap().to_string(),
        json["copyright"].as_str().unwrap().to_string(),
    ))
}
