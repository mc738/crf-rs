use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;

pub struct PackFile {
    pub header: PackFileHeader,
    pub data: File,
}

pub struct PackFileHeader {
    page_size: u32,
    types: HashMap<SupportedAssetType, PackFileTypeIndex>,
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

        let mut buffer = Vec::with_capacity(header_size as usize);

        file.read_exact(&mut buffer).unwrap();

        let header = PackFileHeader::deserialize(
            serde_json::from_str(String::from_utf8(buffer).unwrap().as_str()).unwrap(),
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

                for x in properties
                    .get(stringify!("types"))
                    .unwrap()
                    .as_array()
                    .unwrap()
                {
                    let type_index = PackFileTypeIndex::deserialize(x.clone()).unwrap();
                    types.insert(type_index.asset_type.clone(), type_index);
                }

                Ok(PackFileHeader {
                    page_size: properties
                        .get(stringify!("pageSize"))
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
                    asset_type: SupportedAssetType::Model,
                    page_offset: 0,
                    page_count: 0,
                    size: 0,
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
