use std::{
    fs::{self, File},
    io::Write,
};

use crate::{
    reginald::{coin::Denomination, item::RegiItem, Reginald},
    REGINALD,
};

const PATH: &str = "./bof.json";

pub async fn setup() -> Result<(), std::io::Error> {
    let data = fs::read_to_string(PATH)?;

    if let Ok(regi) = serde_json::from_str::<Reginald>(&data) {
        REGINALD.lock().await.from(regi);
        println!("Finished setup");
    } else {
        eprintln!("Failed setup");
    }

    Ok(())
}

pub async fn desetup() -> Result<(), std::io::Error> {
    let regi = REGINALD.lock().await.clone();
    let mut file = File::create(PATH)?;

    if let Ok(json) = serde_json::to_string(&regi) {
        file.write_all(json.as_bytes())?;
    } else {
        eprintln!("Failed wroting :(");
    }

    Ok(())
}
