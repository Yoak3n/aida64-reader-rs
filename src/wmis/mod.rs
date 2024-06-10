use wmi::*;

use super::Data;
// use std::collections::HashMap;


pub fn read_from_wmi() -> Result<Vec<Data>,Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\WMI",com_con.into())?;
    let results: Vec<Data> = wmi_con.query()?;
    let data = Vec::new();
    for os in results {
        println!("{:?}", os);
    }
    Ok(data)
}
#[cfg(test)]
mod tests;