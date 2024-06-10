#[cfg(feature = "reg")]
pub mod reg;
#[cfg(feature = "shm")]
pub mod shm;
use serde::Deserialize;
#[cfg(feature = "wmis")]
pub mod wmis;

#[derive(Debug, Deserialize)]
#[serde(rename = "AIDA64_SensorValues")]
#[serde(rename_all = "snake_case")]
pub struct Data {
    pub id: String,
    pub label: String,
    pub value: String,
}
