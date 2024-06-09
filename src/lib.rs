#[cfg(feature = "shm")]
pub mod shm;
#[cfg(feature = "reg")]
pub mod registry;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Data {
    pub id: String,
    label: String,
    pub value: String,
}