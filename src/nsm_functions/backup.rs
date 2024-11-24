use crate::{
    input::{get_valid_nsm_save_name, get_yes_or_no}, list::SaveListTraits, nsm_save_data::nsm_save::{is_valid_nsm_save_name, read_nsm_file, NSMFile, NSMSave}
};
use chrono::Local;
use copy_dir::copy_dir;
use std::{
    fs,
    io::{self},
    path::{Path, PathBuf},
};

pub fn backup(
    nsm_save_path: &PathBuf,
    steam_save_path: &PathBuf,
    save_list: &mut Vec<NSMSave>,
) -> Result<(), std::io::Error> {
    // Attempt to copy the save00 folder into the NSM saves folder
    let loaded_save_nsm_file_path = steam_save_path.clone().join(Path::new("nsm.txt"));
    let mut nsm_file: NSMFile = match read_nsm_file(&loaded_save_nsm_file_path) {
        // nsm file already exists, all good
        Ok(file) => file,
        // nsm file does not exist, prompt user to create one
        Err(_) => {
            let mut user_input: String;
            'outer: loop {
                // get input
                user_input = get_valid_nsm_save_name(
                    "What would you like to name this save? "
                );

                // Make sure name isn't already taken by another save
                for save in save_list.iter() {
                    if save.name == user_input {
                        println!(
                            "There is already a save called \"{}\". Please choose a new name.",
                            user_input
                        );
                        continue 'outer;
                    }
                }
                break;
            }

            NSMFile {
                name: user_input,
                save_date: Local::now().into(),
            }
        }
    };

    // update backup date
    nsm_file.save_date = Local::now().into();
    fs::write(loaded_save_nsm_file_path, nsm_file.to_string())?;

    // Make sure name isn't already taken by another save
    for save in &mut *save_list {
        if save.name == nsm_file.name {
            if get_yes_or_no(&format!("There is already a save called \"{}\". Would you like to overwrite? (y/n) ", save.name)) {
                fs::remove_dir_all(save.save_path.clone())?;
                break;
            } else {
                return Ok(());
            }
        }
    }

    // copy save
    let copy_path = nsm_save_path.clone().join(Path::new(&nsm_file.name));
    println!("Backing up save...");
    copy_dir(steam_save_path, copy_path)?;

    // update save list
    save_list.update(nsm_save_path)?;
    Ok(())
}
