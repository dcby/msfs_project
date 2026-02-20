use std::{
    error::Error,
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
};

use quick_xml::{Writer, events::BytesText};

use crate::config::ProjectConfig;

pub fn create_package_definitions<P, C>(root: P, config: &C) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    C: ProjectConfig,
{
    let mut path_buf: PathBuf = root.as_ref().to_path_buf();
    path_buf.push("PackageDefinitions");

    fs::create_dir(&path_buf)?;

    path_buf.push(config.author_and_slug());
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
        .create_element("AssetPackage")
        .with_attribute(("Version", "0.1.0"))
        .write_inner_content(|w| {
            w.create_element("ItemSettings").write_inner_content(|w| {
                w.create_element("ContentType")
                    .write_text_content(BytesText::new("SCENERY"))?;
                w.create_element("Title")
                    .write_text_content(BytesText::new(
                        format!(
                            "{} FSLTL Static (for {})",
                            config.icao().to_owned().to_uppercase(),
                            config.vendor()
                        )
                        .as_str(),
                    ))?;
                w.create_element("Creator")
                    .write_text_content(BytesText::new(config.author()))?;
                Ok(())
            })?;

            w.create_element("Flags").write_inner_content(|w| {
                w.create_element("VisibleInStore")
                    .write_text_content(BytesText::new("false"))?;
                w.create_element("CanBeReferenced")
                    .write_text_content(BytesText::new("false"))?;
                Ok(())
            })?;

            w.create_element("PackageOrderHint")
                .write_text_content(BytesText::new("CUSTOM_AIRPORT_PATCH"))?;

            w.create_element("AssetGroups").write_inner_content(|w| {
                w.create_element("AssetGroup")
                    .with_attribute(("Name", "static"))
                    .write_inner_content(|w| {
                        w.create_element("Type")
                            .with_attribute(("Version", "0"))
                            .write_text_content(BytesText::new("BGL"))?;
                        w.create_element("Flags").write_inner_content(|w| {
                            w.create_element("FSXCompatibility")
                                .write_text_content(BytesText::new("false"))?;
                            Ok(())
                        })?;
                        w.create_element("AssetDir")
                            .write_text_content(BytesText::new("PackageSources\\Scenery\\"))?;
                        w.create_element("OutputDir")
                            .write_text_content(BytesText::new("Scenery\\"))?;
                        Ok(())
                    })?;
                Ok(())
            })?;

            Ok(())
        })?;

    Ok(())
}
