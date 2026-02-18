pub trait AppConfig {
    fn is_force(&self) -> bool;
}

pub trait ProjectConfig {
    fn author(&self) -> &str;
    fn author_and_slug(&self) -> &str;
    fn icao(&self) -> &str;
    fn name(&self) -> &str;
    fn short_vendor(&self) -> &str;
    fn slug(&self) -> &str;
    fn vendor(&self) -> &str;
}

#[derive(Debug)]
pub struct Config {
    author: String,
    author_and_slug: String,
    icao: String,
    is_force: bool,
    name: String,
    short_vendor: String,
    slug: String,
    vendor: String,
}

impl Config {
    pub fn new<A>(author: A, icao: A, is_force: bool, name: A, vendor: A) -> Self
    where
        A: AsRef<str>,
    {
        let author = String::from(author.as_ref()).to_lowercase();
        let icao = String::from(icao.as_ref()).to_lowercase();
        let name = String::from(name.as_ref()).to_lowercase();
        let vendor = String::from(vendor.as_ref());
        let short_vendor = vendor.to_lowercase().replace(" ", "");
        let slug = format!("static-{}-{}-{}", short_vendor, icao, name);
        let author_and_slug = format!("{author}-{slug}");

        Self {
            author,
            author_and_slug,
            icao,
            is_force,
            name,
            short_vendor,
            slug,
            vendor,
        }
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

    fn name(&self) -> &str {
        &self.name
    }

    fn short_vendor(&self) -> &str {
        &self.short_vendor
    }

    fn slug(&self) -> &str {
        &self.slug
    }

    fn vendor(&self) -> &str {
        &self.vendor
    }
}
