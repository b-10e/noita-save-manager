use crate::{
    backup, input::{get_valid_nsm_save_name, get_yes_or_no}, nsm_save_data::nsm_save::{is_valid_nsm_save_name, NSMSave}
};
use copy_dir::copy_dir;
use std::{fs, io, path::PathBuf};

pub fn restore(
    nsm_save_path: &PathBuf,
    steam_save_path: &PathBuf,
    save_list: &mut Vec<NSMSave>,
) -> Result<(), std::io::Error> {
    let mut user_input = String::new();

    let selected_save: NSMSave;
    'outer: loop {
        // get input
        user_input = get_valid_nsm_save_name("Enter the name of the save to restore:");

        // see if given save exists
        for save in save_list.iter() {
            if save.name == user_input {
                selected_save = save.clone();
                break 'outer;
            }
        }

        println!("No such save named \"{}\" exists.", user_input);
    }

    // check if there is an existing steam save
    if steam_save_path.exists() {
        if get_yes_or_no("There is an existing save. Would you like to back it up first? (y/n)") {
            backup::backup(nsm_save_path, steam_save_path, save_list)?;
        }
    }

    // delete the existing save in steam
    fs::remove_dir_all(steam_save_path)?;

    // restore selected save
    println!("Restoring save...");
    copy_dir(selected_save.save_path.clone(), steam_save_path)?;

    Ok(())
}
