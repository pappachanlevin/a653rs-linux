//! The channels used in combination with the related *sampling* module
// TODO: Consider merging this module with sampling, as having a module only
// providing structs might be weird.
use std::collections::HashSet;

use bytesize::ByteSize;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SamplingChannelConfig {
    pub name: String,
    #[serde(deserialize_with = "de_size_str")]
    pub msg_size: ByteSize,
    pub source: String,
    pub destination: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueuingChannelConfig {
    pub name: String,
    #[serde(deserialize_with = "de_size_str")]
    pub msg_size: ByteSize,
    pub msg_num: usize,
    pub source: String,
    pub destination: String,
}

fn de_size_str<'de, D>(de: D) -> Result<ByteSize, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(de)?
        .parse::<ByteSize>()
        .map_err(serde::de::Error::custom)
}
