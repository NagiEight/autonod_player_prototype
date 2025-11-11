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
