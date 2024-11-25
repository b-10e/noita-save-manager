use std::{fs, path::PathBuf};

use crate::nsm_save_data::nsm_save::{read_nsm_file, NSMSave};

pub fn list(nsm_save_path: &PathBuf) -> Result<(), std::io::Error> {
    // Get list of all currently stored saves
    let save_list = get_save_list(&nsm_save_path)?;
    for save in save_list {
        println!(
            "{}\n\tBacked up: {}",
            save.name,
            save.save_date.format("%m/%d/%Y %H:%M")
        );
    }
    println!();
    Ok(())
}

pub fn get_save_list(nsm_save_path: &PathBuf) -> Result<Vec<NSMSave>, std::io::Error> {
    // Attempt to get a list of all the folders in the save directory
    let saves = fs::read_dir(nsm_save_path)?;

    let mut nsm_save_list: Vec<NSMSave> = Vec::new();

    // Loop through every save and push it to save_name_list
    for save_result in saves {
        let save_path = save_result?;
        let nsm_file = read_nsm_file(&save_path.path().join("nsm.txt"))?;
        let nsm_save = NSMSave {
            name: save_path
                .path()
                .file_name()
                .expect("Couldnt get folder name")
                .to_str()
                .expect("Couldnt get folder name")
                .trim()
                .to_string(),
            save_date: nsm_file.save_date,
            save_path: save_path.path(),
        };
        nsm_save_list.push(nsm_save);
    }

    Ok(nsm_save_list)
}

pub trait SaveListTraits {
    fn update(&mut self, nsm_save_path: &PathBuf) -> Result<(), std::io::Error>;
}

impl SaveListTraits for Vec<NSMSave> {
    // Rebuilds the entire save list
    fn update(&mut self, nsm_save_path: &PathBuf) -> Result<(), std::io::Error> {
        // Attempt to get a list of all the folders in the save directory
        let saves = fs::read_dir(nsm_save_path)?;

        // Loop through every save and push it to save_name_list
        for save_result in saves {
            let save_path = save_result?;
            let nsm_file = read_nsm_file(&save_path.path().join("nsm.txt"))?;
            let nsm_save = NSMSave {
                name: save_path
                    .path()
                    .file_name()
                    .expect("Couldnt get folder name")
                    .to_str()
                    .expect("Couldnt get folder name")
                    .trim()
                    .to_string(),
                save_date: nsm_file.save_date,
                save_path: save_path.path(),
            };
            self.push(nsm_save);
        }
        Ok(())
    }
}
