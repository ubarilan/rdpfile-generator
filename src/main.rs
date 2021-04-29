#![allow(non_snake_case, unused_variables)]

mod utils;
#[macro_use]
extern crate colour;
extern crate rpassword;
use std::env;

fn main() {
    let currentpath = env::current_dir().unwrap().display().to_string();
    let mut connection_path: String = utils::get_user_input("[?] Enter the path for the connection file (default connection.rdp): ", true).trim().replace("\n", "");
    if connection_path.trim().is_empty() {
        connection_path = currentpath + "/connection.rdp";//format!("{}/connection.rdp", currentpath);
        blue_ln!("[*] Path has been set to {}", String::from(&connection_path));
    }
    let abs_path = utils::get_abs_path(String::from(&connection_path));
    let isValidPath = utils::validate_path(String::from(&connection_path));
    if !isValidPath {
        std::process::exit(1);
    }

    let ip_address: String = utils::get_user_input("[?] Enter the server's IP address: ", false).replace("\n", "");
    let mut username: String = utils::get_user_input("[?] Enter the username for the server (default Administrator): ", true).replace("\n", "");
    if username.trim().is_empty() {
        username = String::from("Administrator");
        blue_ln!("[*] Username has been set to Administrator");
    }
    let password: String = utils::input_password("Enter password: ");
    let rdp_settings: String = format!("full address:s:{0}\nusername:s:{1}\npassword 51:b:{2}", ip_address, username, password);
    utils::write_rdp_file(rdp_settings, String::from(&connection_path));
    green_ln!("\n[+] Connection file has been saved to {}", abs_path);
}