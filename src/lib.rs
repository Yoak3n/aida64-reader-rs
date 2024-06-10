#[cfg(feature = "shm")]
pub mod shm;
#[cfg(feature = "reg")]
pub mod reg;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Data {
    pub id: String,
    label: String,
    pub value: String,
}