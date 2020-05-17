use colored::Colorize;
use std::option::Option;
use std::string::String;
use winreg::{enums::*, RegKey};

fn set_env(name: String, key: String) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap();
    env.set_value(name, &key).unwrap();
    println!("Result : {}", "Env Set Successfully".green());
}

fn set_reg(name: String, key: String, path_name: String) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey(path_name).unwrap();
    env.set_value(name, &key).unwrap();
    println!("Result : {}", "Registery Set Successfully".green());
}

pub fn set(nam: Option<String>, key: Option<String>) {
    if is_ok(&nam, &key) {
        let name = nam.unwrap().to_string();
        let key = key.unwrap().to_string();
        set_env(name, key);
    }
}

pub fn setr(nam: Option<String>, key: Option<String>, pat: Option<String>) {
    if is_ok(&nam, &key) && is_path(&pat) {
        let name = nam.unwrap().to_string();
        let key = key.unwrap().to_string();
        let path = pat.unwrap().to_string();
        set_reg(name, key, path);
    }
}

fn is_path(p: &Option<String>) -> bool {
    if p.is_none() {
        print!("{}", "error : ".red());
        print!("Paramater Path is not declared\n");
        return false;
    }
    return true;
}

fn is_ok(n: &Option<String>, k: &Option<String>) -> bool {
    if n.is_none() {
        print!("{}", "error : ".red());
        print!("Paramater Name is not declared\n");
        return false;
    }
    if k.is_none() {
        print!("{}", "error : ".red());
        print!("Paramater Key is not declared\n");
        return false;
    }
    return true;
}
