use super::{CODES_FOLDER, qrcode_gen};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct BoxData {
    uuid: String,
    pub box_contents: String,
}

impl BoxData {
    pub fn new(box_contents: String) -> Self {
        let uuid = Uuid::new_v4().to_string();

        let built = Self { uuid, box_contents };

        let qr_code = qrcode_gen::gen_qr_code_for_box(&built);
        qrcode_gen::to_pdf(qr_code, &format!("{}{}.pdf", CODES_FOLDER, built.uuid));

        built
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn get_pdf_path(&self) -> String {
        format!("{}{}.pdf", CODES_FOLDER, self.uuid)
    }
}

pub fn save_database(data: Vec<BoxData>, file: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(&data)?;
    let mut file = File::create(file)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn read_database(file: &str) -> std::io::Result<Vec<BoxData>> {
    if !Path::new(file).exists() {
        return Ok(vec![]);
    }

    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: Vec<BoxData> = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn find_by_uuid<'a>(data: &'a mut Vec<BoxData>, uuid: &str) -> Option<&'a mut BoxData> {
    for b in data {
        if b.uuid() == uuid {
            return Some(b);
        }
    }
    None
}
