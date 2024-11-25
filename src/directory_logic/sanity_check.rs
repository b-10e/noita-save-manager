use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

pub fn create_nsm_data(
    nsm_save_path: &PathBuf,
    nsm_config_path: &PathBuf,
) -> Result<(), std::io::Error> {
    fs::create_dir_all(nsm_save_path)?;
    fs::create_dir_all(nsm_config_path)?;
    Ok(())
}

pub fn noita_steam_path_exists(steam_save_path: &PathBuf) -> Result<(), std::io::Error> {
    match steam_save_path.exists() {
        true => Ok(()),
        false => Err(std::io::Error::new(
            ErrorKind::NotFound,
            "could not find noita save directory",
        )),
    }
}
