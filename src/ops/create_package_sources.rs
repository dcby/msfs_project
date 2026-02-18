use std::{error::Error, path::Path};

use crate::config::ProjectConfig;

pub fn create_package_sources<P, C>(root: P, config: &C) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    C: ProjectConfig,
{
    Ok(())
}

// PackageSources\Scenery\static.xml
// <?xml version="1.0"?>
// <FSData version="9.0">
// </FSData>
