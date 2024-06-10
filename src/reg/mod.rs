use super::Data;
use std::io;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

pub fn read_from_reg() -> io::Result<Vec<Data>> {
    let aida64_sensor_values = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("Software\\FinalWire\\AIDA64\\SensorValues")?;
    let mut datas = Vec::new();
    for (name, value) in aida64_sensor_values.enum_values().map(|x| x.unwrap()) {
        let field = name.split(".").collect::<Vec<&str>>()[1];
        match datas.iter().position(|x: &Data| x.id == field.to_string()) {
            Some(p) => {
                if name.as_str().starts_with("Value.") {
                    datas[p].value = value.to_string();
                } else if name.as_str().starts_with("Label.") {
                    datas[p].label = value.to_string();
                }
            }
            None => {
                let mut data = Data {
                    id: field.to_string(),
                    label: "".to_string(),
                    value: "".to_string(),
                };
                if name.as_str().starts_with("Value.") {
                    data.value = value.to_string();
                } else if name.as_str().starts_with("Label.") {
                    data.label = value.to_string();
                }
                datas.push(data);
            }
        }
    }
    Ok(datas)
}
