const NOITA_STEAM_SAVE_DIR_SUFFIX: &'static str = ".steam/steam/steamapps/compatdata/881100/pfx/drive_c/users/steamuser/AppData/LocalLow/Nolla_Games_Noita/save00";
const NSM_SAVE_DIR_SUFFIX: &'static str = ".local/share/noitasave/saves";
const NSM_CONFIG_DIR_SUFFIX: &'static str = ".local/share/noitasave/config";

use homedir::my_home;
use std::{
    io::Error,
    path::{Path, PathBuf},
};

fn get_home_path() -> Result<PathBuf, Error> {
    let home_path = match my_home() {
        Ok(home_path_option) => match home_path_option {
            Some(home_path) => home_path,
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not get home directory",
                ))
            }
        },
        Err(e) => return Err(Error::new(std::io::ErrorKind::NotFound, e.to_string())),
    };
    Ok(home_path)
}

pub fn get_nsm_save_path() -> Result<PathBuf, Error> {
    let home_path = get_home_path()?;
    let nsm_save_path = home_path.join(Path::new(NSM_SAVE_DIR_SUFFIX));
    Ok(nsm_save_path)
}

pub fn get_noita_save_path() -> Result<PathBuf, Error> {
    let home_path = get_home_path()?;
    let noita_save_path = home_path.join(Path::new(NOITA_STEAM_SAVE_DIR_SUFFIX));
    Ok(noita_save_path)
}

pub fn get_nsm_config_path() -> Result<PathBuf, Error> {
    let home_path = get_home_path()?;
    let nsm_config_path = home_path.join(Path::new(NSM_CONFIG_DIR_SUFFIX));
    Ok(nsm_config_path)
}
