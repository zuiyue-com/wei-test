extern crate wmi;
extern crate serde;

use wmi::{COMLibrary, Variant, WMIConnection, WMIDateTime};
use serde::Deserialize;

#[derive(Deserialize)]
struct Win32_DiskDrive {
    Caption: String,
    InterfaceType: String,
}

fn main() {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();

    let results: Vec<Win32_DiskDrive> = wmi_con.raw_query("SELECT Caption, InterfaceType FROM Win32_DiskDrive").unwrap();

    for drive in results {
        println!("Caption: {}, InterfaceType: {}", drive.Caption, drive.InterfaceType);
    }
}