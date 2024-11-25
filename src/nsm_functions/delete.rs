use crate::{list::SaveListTraits, nsm_save_data::nsm_save::NSMSave};
use std::{fs, path::PathBuf};

use super::input::{get_valid_nsm_save_name, get_yes_or_no};

pub fn delete(nsm_save_path: &PathBuf, save_list: &mut Vec<NSMSave>) -> Result<(), std::io::Error> {
    loop {
        let user_input = get_valid_nsm_save_name("Enter the name of the save to delete: ");
        for save in save_list.iter() {
            if save.name == user_input {
                if get_yes_or_no("Are you sure you would like to delete this save? (y/n)") {
                    fs::remove_dir_all(save.save_path.clone())?;
                    save_list.update(nsm_save_path)?;
                }
                return Ok(());
            }
        }
        println!("No save with that name exists.")
    }
}
