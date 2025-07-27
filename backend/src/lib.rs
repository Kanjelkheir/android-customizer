use serde::{Serialize};
use serde_json::json;

pub mod utils;


#[derive(Serialize)]
pub struct DeviceInfo {
    #[serde(rename = "ADB Tools Folder")]
    pub adb_tools_folder: String,
    #[serde(rename = "SHRP Recovery")]
    pub shrp_recovery: String,
    #[serde(rename = "Odin Tool")]
    pub odin_tool: String,
    #[serde(rename = "RP .tar image")]
    pub rp_tar_image: String,
    #[serde(rename = "OrangeFox Recovery")]
    pub orange_fox_recovery: String,
    #[serde(rename = "Custom ROM")]
    pub custom_rom: String,
    #[serde(rename = "Stock Firmware")]
    pub stock_firmware: String,
}
