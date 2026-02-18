use std::{
    error::Error,
    fs::{File, create_dir},
    io::BufWriter,
    path::{Path, PathBuf},
};

use quick_xml::Writer;

use crate::config::ProjectConfig;

pub fn create_package_definitions<P, C>(root: P, config: &C) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    C: ProjectConfig,
{
    let mut path_buf: PathBuf = root.as_ref().to_path_buf();
    path_buf.push("PackageDefinitions");

    create_dir(&path_buf)?;

    path_buf.push(config.author_and_slug());
    path_buf.add_extension("xml");

    let file = File::create_new(path_buf)?;
    write(&file, config)?;

    Ok(())
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
        .write_empty()?;

    Ok(())
}

// <?xml version="1.0" encoding="utf-8"?>
// <AssetPackage Version="0.1.0">
// 	<ItemSettings>
// 		<ContentType>SCENERY</ContentType>
// 		<Title>KLIT FSLTL Static (for Propair Flight)</Title>
// 		<Manufacturer/>
// 		<Creator>dcby13</Creator>
// 	</ItemSettings>
// 	<Flags>
// 		<VisibleInStore>false</VisibleInStore>
// 		<CanBeReferenced>false</CanBeReferenced>
// 	</Flags>
// 	<PackageOrderHint>CUSTOM_AIRPORT_PATCH</PackageOrderHint>
// 	<AssetGroups>
// 		<AssetGroup Name="static">
// 			<Type Version="0">BGL</Type>
// 			<Flags>
// 				<FSXCompatibility>false</FSXCompatibility>
// 			</Flags>
// 			<AssetDir>PackageSources\Scenery\</AssetDir>
// 			<OutputDir>Scenery\</OutputDir>
// 		</AssetGroup>
// 	</AssetGroups>
// </AssetPackage>
