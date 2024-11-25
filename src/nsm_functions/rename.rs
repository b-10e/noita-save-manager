use crate::{
    input::get_valid_nsm_save_name,
    nsm_save_data::nsm_save::{NSMFile, NSMSave},
};
use std::{fs, path::PathBuf};

use super::list::SaveListTraits;

pub fn rename(nsm_save_path: &PathBuf, save_list: &mut Vec<NSMSave>) -> Result<(), std::io::Error> {
    let selected_save: NSMSave;
    'outer: loop {
        let user_input = get_valid_nsm_save_name("Enter the name of the save you want to rename: ");
        for save in save_list.iter() {
            if save.name == user_input {
                selected_save = save.clone();
                break 'outer;
            }
        }
        println!("No save with that name exists.")
    }
    // get new name
    let new_name = get_valid_nsm_save_name("Enter the new name: ");

    // update nsm file in save
    let mut new_nsm_file: NSMFile = selected_save.to_nsm_file();
    new_nsm_file.name = new_name.clone();
    fs::write(
        selected_save.save_path.join("nsm.txt"),
        new_nsm_file.to_string(),
    )?;

    // rename save directory name
    let new_name_path = nsm_save_path.join(new_name);
    fs::rename(selected_save.save_path, new_name_path)?;
    save_list.update(nsm_save_path)?;
    Ok(())
}
