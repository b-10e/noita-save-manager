mod directory_logic;
mod nsm_functions;
mod nsm_save_data;

use std::{
    io::{self, stdout, Error, Write},
    path::PathBuf,
};

use list::get_save_list;
use nsm_functions::*;

use directory_logic::{get_dirs::*, sanity_check};
use nsm_save_data::nsm_save::NSMSave;
//use restore::restore;

fn main() -> Result<(), Error> {
    // nsm save directory - nsm_save_path
    // nsm config directory - nsm_config_path
    // noita save directory - steam_save_path

    let nsm_save_path = get_nsm_save_path()?;
    let nsm_config_path = get_nsm_config_path()?;
    let steam_save_path = get_noita_save_path()?;
    
    // ensure everything exists correctly
    sanity_check::create_nsm_data(&nsm_save_path, &nsm_config_path)?;
    match sanity_check::noita_steam_path_exists(&steam_save_path) {
        Ok(_) => (),
        Err(_) => return Err(Error::new(std::io::ErrorKind::NotFound, "Could not find Noita installation!"))
    }

    // create save list
    let mut save_list = get_save_list(&nsm_save_path)?;

    // start nsm
    run(&nsm_save_path, &steam_save_path, &mut save_list)
}

fn run(
    nsm_save_path: &PathBuf,
    steam_save_path: &PathBuf,
    save_list: &mut Vec<NSMSave>,
) -> Result<(), Error> {
    println!("Type \"help\" to see available commands.");
    //todo!("List backed up saves");
    println!();

    // Main loop
    loop {
        // Get user input
        print!("> ");
        stdout().flush().unwrap();
        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input)?;
        let args: Vec<String> = user_input
            .split_whitespace()
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        // Make sure the user actually entered something
        if args.len() == 0 {
            continue;
        }

        handle_input(&args, nsm_save_path, steam_save_path, save_list)?;
    }
}

fn handle_input(
    args: &Vec<String>,
    nsm_save_path: &PathBuf,
    steam_save_path: &PathBuf,
    save_list: &mut Vec<NSMSave>,
) -> Result<(), Error> {
    //let len = args.len();
    match args[0].as_str() {
        "help" => {
            help::help();
            Ok(())
        }
        "list" => list::list(nsm_save_path),
        "backup" => backup::backup(nsm_save_path, steam_save_path, save_list),
        "restore" => restore::restore(nsm_save_path, steam_save_path, save_list),
        "rename" => rename::rename(nsm_save_path, save_list),
        "delete" => delete::delete(nsm_save_path, save_list),
        "quit" => Err(std::io::Error::new(io::ErrorKind::Other, "quit")),
        _ => Ok(()),
    }
}
