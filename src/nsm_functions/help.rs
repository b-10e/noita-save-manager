pub fn help() {
    // List all commands
    println!();
    println!("Available Commands:");
    println!("list ------------------- Lists all backed up saves");
    println!("backup ----------------- Copies the save currently in Noita to the backups folder");
    println!("restore ---------------- Copies the given save into the Noita folder");
    println!("rename ----------------- Renames the given save to the provided string");
    println!("delete ----------------- Deletes the given save");
    println!("quit ------------------- Close Noita Save Manager");
    println!("Noita Save Manager files are stored at ~/.local/share/noitasave/");
    println!();
}
