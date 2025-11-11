use std::process::Command;
use std::{ffi::OsStr, fs::File};
use sysinfo::System;
pub fn check_process_running(process: &str) -> bool {
    //println!("Checking if process '{}' is running...", process);
    return get_process_running(process);
}

fn get_process_running(process_name: &str) -> bool {
    let system = System::new_all();
    let converted_process_name: &OsStr = &OsStr::new(process_name);

    let is_running = system
        .processes_by_name(converted_process_name)
        .next()
        .is_some();

    if is_running {
        return true;
    } else {
        return false;
    }
}

pub fn execute_from_string(command: &str) {
    let result = Command::new(command).spawn();

    match result {
        Ok(_) => println!("executed command: {}", command),
        Err(e) => eprintln!("Failed to launch Waterfox: {}", e),
    }
}

pub fn merge_string<T: AsRef<str>>(parts: &[T]) -> String {
    parts.iter().map(|s| s.as_ref()).collect::<String>()
}

pub fn merge_optional_strings(parts: &[Option<&str>]) -> String {
    parts.iter().filter_map(|opt| *opt).collect::<String>()
}
