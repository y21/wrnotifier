use std::borrow::Cow;

use serde::Deserialize;
use serde::Deserializer;

use crate::characters::CHARACTERS;
use crate::vehicles::VEHICLES;

#[derive(Debug, PartialEq)]
pub enum EngineClass {
    E150,
    E200,
}

fn deserialize_engine_class<'de, D: Deserializer<'de>>(de: D) -> Result<EngineClass, D::Error> {
    match bool::deserialize(de)? {
        true => Ok(EngineClass::E200),
        false => Ok(EngineClass::E150),
    }
}

fn deserialize_character<'de, D: Deserializer<'de>>(de: D) -> Result<&'static str, D::Error> {
    let id = usize::deserialize(de)?;
    CHARACTERS
        .get(id)
        .ok_or_else(|| serde::de::Error::custom(format!("invalid character ID: {id}")))
        .copied()
}

fn deserialize_vehicle<'de, D: Deserializer<'de>>(de: D) -> Result<&'static str, D::Error> {
    let id = usize::deserialize(de)?;
    VEHICLES
        .get(id)
        .ok_or_else(|| serde::de::Error::custom(format!("invalid vehicle ID: {id}")))
        .copied()
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WrEntry<'a> {
    pub track_name: &'a str,
    #[serde(borrow)]
    pub player: Cow<'a, str>,
    #[serde(rename = "200cc", deserialize_with = "deserialize_engine_class")]
    pub engine_class: EngineClass,
    pub finish_time_simple: &'a str,
    pub best_split_simple: &'a str,
    #[serde(rename = "driverId", deserialize_with = "deserialize_character")]
    pub character: &'a str,
    #[serde(rename = "vehicleId", deserialize_with = "deserialize_vehicle")]
    pub vehicle: &'a str,
    pub hash: &'a str,
    pub href: &'a str,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Results<'a> {
    #[serde(borrow)]
    pub recent_records: Vec<WrEntry<'a>>,
}
