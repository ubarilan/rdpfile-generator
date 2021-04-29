#![allow(non_snake_case)]

pub fn get_user_input(prompt: &str, allow_empty: bool) -> String {
    use std::io::{Write};
    yellow!("{}", prompt);
    std::io::stdout().flush().expect("[-] Error while flushing stdout");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("[-] Error handling input");
    if !allow_empty && input.trim().is_empty() {
        red_ln!("[-] Empty input is not allowed");
        std::process::exit(1);
    }
    return input;
}

pub fn input_password(prompt: &str) -> String {
    use std::io::{Write};
    yellow!("{}", prompt);
    std::io::stdout().flush().expect("[-] Error while flushing stdout");
    let password = rpassword::read_password().unwrap();
    if password.trim().is_empty() {
        red_ln!("[-] Empty password is not allowed");
        std::process::exit(1);
    }
    return password;
}

pub fn write_rdp_file(settings: String, write_to: String) {
    use std::fs;
    fs::write(write_to, settings).expect("[-] Unable to write to file");
}

pub fn get_abs_path(path: String) -> String {
    use std::path::Path;
    use path_absolutize::*;
    let fsPath = Path::new(&path);
    let absolute_path = fsPath.absolutize().unwrap();
    let x = absolute_path.to_str().unwrap();
    return x.to_string();
}

pub fn validate_path(path: String) -> bool {
    use std::path::Path;
    use path_absolutize::*;
    let fsPath = Path::new(&path);
 
    let absolute_path = fsPath.absolutize().unwrap();
    let x = absolute_path.to_str().unwrap();
    if absolute_path.is_dir() {
        red_ln!("[-] Given path is a directory");
        std::process::exit(1);
    }
    let exists = absolute_path.parent() != None && absolute_path.parent().unwrap().exists();
    if !exists {
        red_ln!("[-] Given path does not exist");
        std::process::exit(1);
    }
    return true;
}