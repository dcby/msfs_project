use std::{
    error::Error,
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use quick_xml::{Writer, events::BytesText};

use crate::config::ProjectConfig;

pub fn create_project<P, C>(root: P, config: &C) -> Result<(), Box<dyn Error>>
where
    C: ProjectConfig,
    P: AsRef<Path>,
{
    let mut path_buf: PathBuf = root.as_ref().to_path_buf();
    path_buf.push(config.slug());
    path_buf.add_extension("xml");

    let file = File::create_new(path_buf)?;

    write(&file, config)
}

fn write<C>(file: &File, config: &C) -> Result<(), Box<dyn Error>>
where
    C: ProjectConfig,
{
    let mut writer = Writer::new_with_indent(BufWriter::new(file), b'\t', 1);
    writer.write_event(quick_xml::events::Event::Decl(
        quick_xml::events::BytesDecl::new("1.0", Some("utf-8"), None),
    ))?;

    writer
        .create_element("Project")
        .with_attributes(vec![
            ("Version", "2"),
            ("Name", config.slug()),
            ("FolderName", "Packages"),
            ("MetadataFolderName", "PackagesMetadata"),
        ])
        .write_inner_content(|w| {
            w.create_element("OutputDirectory")
                .write_text_content(BytesText::new("."))?;
            w.create_element("TemporaryOutputDirectory")
                .write_text_content(BytesText::new("_PackageInt"))?;
            w.create_element("Packages").write_inner_content(|w| {
                w.create_element("Package")
                    .write_text_content(BytesText::new(
                        format!("PackageDefinitions\\{}.xml", config.author_and_slug()).as_str(),
                    ))?;
                Ok(())
            })?;

            Ok(())
        })?;

    Ok(())
}
