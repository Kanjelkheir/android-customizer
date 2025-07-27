use std::collections::HashMap;
use crate::DeviceInfo;

pub fn parse_json(content: String) -> String {
    let mut map = HashMap::new();
    let mut titles = Vec::new();
    let mut links = Vec::new();
    let mut output = format!("{{\n");
    let content: Vec<String> = content.lines().map(|line| line.to_string())
        .collect();
    for line in content {
        if line.contains("Link:") {
            links.push(line.replace("Link: ", "").trim().split("").collect::<Vec<&str>>()[1].to_string());
        } else {
            titles.push(line.split(". ").collect::<Vec<&str>>()[0].to_string());
        }
    }
    for i in 0..links.len() {
        map.insert(titles[i].clone(), links[i].clone());
    }
    for (key, value) in map {
        output.push_str(&format!("\"{}\": \"{}\",\n", key, value));
    }
    output.push_str("}");
    output
}


pub fn json_parse(content: String) -> DeviceInfo {
    let content: Vec<String> = content.lines().map(|line| line.to_string()).collect();
    let mut links: Vec<String> = Vec::new();
    for line in content {
        if line.contains("Link: ") {
            links.push(line.replace("Link: ", "").split(" ").collect::<Vec<&str>>()[0].to_string());
        }
    }

    DeviceInfo {
        adb_tools_folder: links[0].clone(),
        shrp_recovery: links[1].clone(),
        odin_tool: links[2].clone(),
        rp_tar_image: links[3].clone(),
        orange_fox_recovery: links[4].clone(),
        custom_rom: links[5].clone(),
        stock_firmware: links[6].clone(),
    }
}
