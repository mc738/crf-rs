use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use uuid::Uuid;

pub struct PackFile {
    pub header: PackFileHeader,
    pub data: File,
}

pub struct PackFileHeader {
    page_size: u32,
    pub types: HashMap<SupportedAssetType, PackFileTypeIndex>,
}

pub struct PackFileTypeIndex {
    asset_type: SupportedAssetType,
    page_offset: u32,
    page_count: u32,
    size: u64,
    items: HashMap<Uuid, PackFileItemIndex>,
}

pub struct PackFileItemIndex {
    id: Uuid,
    index: u32,
    asset_block_offset: u64,
    length: u32,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum SupportedAssetType {
    Model,
    Texture,
}

impl PackFile {
    pub fn load_from_file(path: &str) -> PackFile {
        let mut file = File::open(path).unwrap();

        let mut header_buffer = [0u8; 20];

        // read the magic bytes
        file.read(&mut header_buffer).unwrap();

        let magic_bytes = String::from_utf8(header_buffer[0..4].to_vec()).unwrap();

        let version = u32::from_le_bytes(header_buffer[4..8].try_into().unwrap());

        // TODO version match

        let page_size = u32::from_le_bytes(header_buffer[8..12].try_into().unwrap());

        let header_pages = u32::from_le_bytes(header_buffer[12..16].try_into().unwrap());

        let header_size = u32::from_le_bytes(header_buffer[16..20].try_into().unwrap());

        let p1 = file.stream_position().unwrap();

        //let mut buffer = Vec::with_capacity(header_size as usize);
        //let mut b = [ 0; 4096 - 20 ];

        //let writeCount = file.read(&mut b).unwrap();

        //let p2 = file.stream_position

        let mut v = vec![0u8; header_size as usize];
        file.read_exact(&mut v).unwrap();

        let header = PackFileHeader::deserialize(
            serde_json::from_str::<serde_json::Value>(String::from_utf8(v).unwrap().as_str()).unwrap(),
        )
        .unwrap();

        PackFile {
            header,
            data: file,
        }
    }
}

impl PackFileHeader {
    pub fn deserialize(root: Value) -> Result<PackFileHeader, &'static str> {
        match root {
            Value::Null => Err("Invalid PackFileHeader"),
            Value::Bool(_) => Err("Invalid PackFileHeader"),
            Value::Number(_) => Err("Invalid PackFileHeader"),
            Value::String(_) => Err("Invalid PackFileHeader"),
            Value::Array(_) => Err("Invalid PackFileHeader"),
            Value::Object(properties) => {
                let mut types = HashMap::new();

                //for x in &properties {
                //    println!("{:?}", x)
                //}

                let t = properties.get("types").unwrap();


                for x in
                    t
                    .as_array()
                    .unwrap()
                {
                    let l = x;
                    println!("{:?}", l);
                    let type_index = PackFileTypeIndex::deserialize(x.clone()).unwrap();
                    types.insert(type_index.asset_type.clone(), type_index);
                }

                Ok(PackFileHeader {
                    page_size: properties
                        .get("pageSize")
                        .unwrap()
                        .as_u64()
                        .unwrap() as u32,
                    types,
                })
            }
        }
    }
}

impl PackFileTypeIndex {
    pub fn deserialize(value: Value) -> Result<PackFileTypeIndex, &'static str> {
        match value {
            Value::Null => Err("Null value provided for PackFileTypeIndex"),
            Value::Bool(_) => Err("Boolean value provided for PackFileTypeIndex"),
            Value::Number(_) => Err("Number value provided for PackFileTypeIndex"),
            Value::String(_) => Err("String value provided for PackFileTypeIndex"),
            Value::Array(_) => Err("Array value provided for PackFileTypeIndex"),
            Value::Object(typeProperties) => {
                let mut items = HashMap::new();

                for i in typeProperties.get("items").unwrap().as_array().unwrap() {
                    let item = PackFileItemIndex::deserialize(i.clone()).unwrap();
                    items.insert(item.id, item);
                }

                Ok(PackFileTypeIndex {
                    // TODO fix this
                    asset_type: SupportedAssetType::deserialize(typeProperties.get("type").unwrap()).unwrap(),
                    page_offset: typeProperties.get("pageOffset").unwrap().as_u64().unwrap() as u32,
                    page_count: typeProperties.get("pageCount").unwrap().as_u64().unwrap() as u32,
                    size: typeProperties.get("size").unwrap().as_u64().unwrap(),
                    items,
                })
            }
        }
    }
}

impl PackFileItemIndex {
    pub fn deserialize(value: Value) -> Result<PackFileItemIndex, &'static str> {
        match value {
            Value::Null => Err("Null value provided for PackFileTypeIndex"),
            Value::Bool(_) => Err("Boolean value provided for PackFileTypeIndex"),
            Value::Number(_) => Err("Number value provided for PackFileTypeIndex"),
            Value::String(_) => Err("String value provided for PackFileTypeIndex"),
            Value::Array(_) => Err("Array value provided for PackFileTypeIndex"),
            Value::Object(itemProperties) => Ok(PackFileItemIndex {
                id: Uuid::try_parse(itemProperties.get("id").unwrap().as_str().unwrap()).unwrap(),
                index: itemProperties.get("index").unwrap().as_u64().unwrap() as u32,
                asset_block_offset: itemProperties
                    .get("assetBlockOffset")
                    .unwrap()
                    .as_u64()
                    .unwrap(),
                length: itemProperties.get("length").unwrap().as_u64().unwrap() as u32,
            }),
        }
    }
}


impl SupportedAssetType {

    pub fn deserialize(value: &Value) -> Result<SupportedAssetType, &'static str> {
        match value {
            Value::Null => Err("Null value provided for SupportedAssetType"),
            Value::Bool(_) => Err("Boolean value provided for SupportedAssetType"),
            Value::Number(_) => Err("Number value provided for SupportedAssetType"),
            Value::String(v) =>
                match v.as_str() {
                    "model" => Ok(SupportedAssetType::Model),
                    "texture" => Ok(SupportedAssetType::Texture),
                    _ => Err("Unknown value provided for SupportedAssetType"),
                }
            Value::Array(_) => Err("Array value provided for SupportedAssetType"),
            Value::Object(itemProperties) => Ok(SupportedAssetType::Model),
        }
    }
}