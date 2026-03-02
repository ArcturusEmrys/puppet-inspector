use inox2d::formats::inp::parse_inp_parts;
use inox2d::model::{ModelTexture, VendorData};
use inox2d::puppet::Puppet;
use json::JsonValue;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

mod controller;
pub use controller::DocumentController;

pub struct Document {
    puppet_json: JsonValue,
    pub(crate) puppet_data: Puppet,
    textures: Vec<ModelTexture>,
    vendors: Vec<VendorData>,
}

impl Document {
    pub fn open(file: impl Read) -> Result<Self, Box<dyn Error>> {
        let (puppet_json, textures, vendors) = parse_inp_parts(file)?;
        let puppet_data = Puppet::new_from_json(&puppet_json)?;

        Ok(Self {
            puppet_json,
            puppet_data,
            textures,
            vendors,
        })
    }
}
