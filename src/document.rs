use std::path::{Path, PathBuf};
use std::fs::File;
use std::error::Error;
use std::io::Read;
use inox2d::puppet::Puppet;
use inox2d::model::{ModelTexture, VendorData};
use inox2d::formats::inp::parse_inp_parts;
use json::JsonValue;

pub struct Document {
    puppet_json: JsonValue,
    puppet_data: Puppet,
    textures: Vec<ModelTexture>,
    vendors: Vec<VendorData>
}

impl Document {
    pub fn open(file: impl Read) -> Result<Self, Box<dyn Error>> {
        let (puppet_json, textures, vendors) = parse_inp_parts(file)?;
        let puppet_data = Puppet::new_from_json(&puppet_json)?;

        Ok(Self {
            puppet_json,
            puppet_data,
            textures,
            vendors
        })
    }
}