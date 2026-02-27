use regex_lite::Regex;

pub trait AppConfig {
    fn is_force(&self) -> bool;
}

pub trait ProjectConfig {
    fn author_and_slug(&self) -> &str;
    fn author(&self) -> &str;
    fn icao(&self) -> &str;
    fn name_slug(&self) -> &str;
    fn path(&self) -> Option<&str>;
    fn slug(&self) -> &str;
    fn vendor_code(&self) -> &str;
    fn vendor(&self) -> &str;
}

#[derive(Debug)]
pub struct Config {
    author_and_slug: String,
    author: String,
    icao: String,
    is_force: bool,
    name_slug: String,
    path: Option<String>,
    slug: String,
    vendor_code: String,
    vendor: String,
}

impl Config {
    pub fn new(
        author: impl Into<String>,
        icao: impl Into<String>,
        is_force: bool,
        name: impl Into<String>,
        name_slug: Option<impl Into<String>>,
        path: Option<impl Into<String>>,
        vendor: impl Into<String>,
        vendor_code: Option<impl Into<String>>,
    ) -> Self {
        let author = author.into().to_lowercase();
        let icao = icao.into().to_lowercase();
        let name = name.into();
        let name_slug = match name_slug {
            Some(s) => s.into().to_lowercase(),
            None => Self::get_name_slug(&name),
        };
        let vendor = vendor.into();
        let vendor_code = match vendor_code {
            Some(s) => s.into().to_lowercase(),
            None => Self::get_vendor_code(&vendor),
        };
        let slug = format!("static-{}-{}-{}", vendor_code, icao, name_slug);
        let author_and_slug = format!("{author}-{slug}");

        Self {
            author,
            author_and_slug,
            icao,
            is_force,
            name_slug,
            path: path.map(|e| e.into()),
            vendor_code,
            slug,
            vendor,
        }
    }

    fn get_name_slug(value: &str) -> String {
        let re = Regex::new(r"[^a-z0-9\-]").unwrap();
        let s = value.to_lowercase().replace(" ", "-");
        re.replace_all(&s, "").into()
    }

    fn get_vendor_code(value: &str) -> String {
        let re = Regex::new(r"[^a-z0-9]").unwrap();
        let s = value.to_lowercase();
        re.replace_all(&s, "").into()
    }
}

impl AppConfig for Config {
    fn is_force(&self) -> bool {
        self.is_force
    }
}

impl ProjectConfig for Config {
    fn author(&self) -> &str {
        &self.author
    }

    fn author_and_slug(&self) -> &str {
        &self.author_and_slug
    }

    fn icao(&self) -> &str {
        &self.icao
    }

    fn name_slug(&self) -> &str {
        &self.name_slug
    }

    fn path(&self) -> Option<&str> {
        self.path.as_ref().map(|e| e.as_str())
    }

    fn slug(&self) -> &str {
        &self.slug
    }

    fn vendor(&self) -> &str {
        &self.vendor
    }

    fn vendor_code(&self) -> &str {
        &self.vendor_code
    }
}
