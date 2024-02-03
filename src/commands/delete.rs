use std::fs;
use std::io;
use std::path::Path;

fn verify_path(file: &Path) -> bool {
    if !file.exists() {
        return false;
    }
    // whats better than one existence check? two existence checks!
    file.try_exists()
        .expect("File existence could not be verified");
    return true;
}

pub fn delete(file: &Path) {
    if !verify_path(file) {
        return;
    }

    if file.is_dir() {
        let mut ans = String::new();
        println!("Remove ALL directory contents? (Y/N): ");
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read line");
        if ans.to_lowercase() == "y" {
            // user wants to remove all dir contents
            println!(
                "Remove this directory and ALL contents? This operation is NOT reversable (Y/N): "
            );
            io::stdin()
                .read_line(&mut ans)
                .expect("Failed to read line");

            if ans.to_lowercase() == "y" {
                fs::remove_dir_all(file).expect("Unable to remove all file contents");
                println!("Removed directory '{}' and all contents", file.display());
            }
        } else if ans.to_lowercase() == "n" {
            // user wants to remove empty dir
            println!("Remove this empty directory? This operation is NOT reversable (Y/N): ");
            io::stdin()
                .read_line(&mut ans)
                .expect("Failed to read line");

            if ans.to_lowercase() == "y" {
                fs::remove_dir(file).expect("Unable to remove all file contents");
                println!("Removed empty directory '{}'", file.display());
            }
        }
    } else if file.is_file() {
        let mut ans = String::new();
        println!("Remove this file? This operation is NOT reversable (Y/N): ");
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read line");

        if ans.to_lowercase() == "y" {
            fs::remove_file(file).expect("Unable to remove file");
            println!("Removed file '{}'", file.display());
        }
    } else if file.is_symlink() {
        todo!();
    }
}
