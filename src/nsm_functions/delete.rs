use crate::{
    list::SaveListTraits,
    nsm_save_data::nsm_save::{is_valid_nsm_save_name, read_nsm_file, NSMFile, NSMSave},
};
use std::{
    fs,
    io::{self, Error},
    path::{Path, PathBuf},
};

pub fn delete(
    nsm_save_path: &PathBuf,
    steam_save_path: &PathBuf,
    save_list: &mut Vec<NSMSave>,
) -> Result<(), std::io::Error> {
    println!("Enter the name of the save to delete: ");

    Ok(())
}
