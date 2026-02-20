use std::{
    error::Error,
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
};

use quick_xml::Writer;

pub fn create_package_sources<P>(root: P) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut path_buf: PathBuf = root.as_ref().to_path_buf();
    path_buf.push("PackageSources\\Scenery");

    fs::create_dir_all(&path_buf)?;

    path_buf.push("static.xml");

    let file = File::create_new(path_buf)?;
    write(&file)
}

fn write(file: &File) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::new_with_indent(BufWriter::new(file), b'\t', 1);
    writer.write_event(quick_xml::events::Event::Decl(
        quick_xml::events::BytesDecl::new("1.0", None, None),
    ))?;

    writer
        .create_element("FSData")
        .with_attribute(("version", "9.0"))
        .write_empty()?;

    Ok(())
}
