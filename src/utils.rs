use crate::{
    constants::{CHIP8_ARCHIVE_RAW_URL, CHIP8_ARCHIVE_URL, TICKRATE},
    rom::Rom,
    types::Result,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    io::{Cursor, Error, Read},
    path::PathBuf,
};

pub struct Utils;

impl Utils {
    pub fn exe_dir() -> std::result::Result<PathBuf, Error> {
        let mut path = std::env::current_exe()?;
        path.pop();
        Ok(path)
    }

    pub fn roms_dir() -> std::result::Result<PathBuf, Error> {
        let mut path = Self::exe_dir()?;
        path.push("roms");
        Ok(path)
    }

    pub fn download_rom(name: &str) -> Result<()> {
        let mut rom_path = Utils::roms_dir()?;

        if !rom_path.exists() {
            std::fs::create_dir(&rom_path)?;
        }

        rom_path.push(format!("{name}.ch8"));

        if rom_path.exists() {
            return Ok(());
        } else {
            let response =
                reqwest::blocking::get(format!("{CHIP8_ARCHIVE_URL}/raw/master/roms/{name}.ch8"));
            let mut file = std::fs::File::create(rom_path)?;
            let mut content = Cursor::new(response?.bytes()?);

            std::io::copy(&mut content, &mut file)?;
        }

        Ok(())
    }

    pub fn fetch_rom_list() -> Result<Vec<Rom>> {
        let mut json_file = Utils::exe_dir()?;
        json_file.push("roms.json");

        let mut roms: Vec<Rom> = Vec::with_capacity(43);

        if !json_file.exists() {
            let body =
                reqwest::blocking::get(format!("{CHIP8_ARCHIVE_RAW_URL}/master/programs.json"))?
                    .text()?;
            let json: HashMap<String, Value> = serde_json::from_str(&body)?;

            let mut filtered_roms: Vec<Rom> = json
                .iter()
                .filter(|(_name, item)| item["platform"].to_string().contains("chip8"))
                .map(Rom::from)
                .collect();

            roms.append(&mut filtered_roms);
            std::fs::write(json_file, serde_json::to_string_pretty(&roms)?.as_bytes())?;

            return Ok(roms);
        }

        let mut file = std::fs::File::open(json_file)?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let mut deserialized_roms = serde_json::from_str(&file_contents)?;
        roms.append(&mut deserialized_roms);

        Ok(roms)
    }

    pub fn downloaded_roms() -> Result<Vec<String>> {
        let mut roms: Vec<String> = Vec::with_capacity(43);
        let roms_dir = Self::roms_dir()?;

        if let Ok(files) = std::fs::read_dir(roms_dir) {
            for file in files.flatten() {
                if let Some(filename) = file.file_name().to_str() {
                    roms.push(filename.replace(".ch8", ""));
                }
            }
        }

        Ok(roms)
    }

    pub fn find_rom(name: &str) -> Result<Rom> {
        let mut program_file = Self::exe_dir()?;
        program_file.push("roms.json");

        let mut file = std::fs::File::open(program_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let deserialized_roms: Vec<Rom> = serde_json::from_str(&contents)?;

        let rom = deserialized_roms
            .iter()
            .find(|&rom| rom.title == name)
            .cloned();

        if rom.is_none() {
            return Err("Could not find rom!".into());
        }

        Ok(rom.unwrap())
    }

    pub fn instruction_time_ns() -> u128 {
        1e9 as u128 / TICKRATE
    }
}
