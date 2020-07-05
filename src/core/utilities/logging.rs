use std::{fs, mem};
use winapi::ctypes::c_char;
use crate::core::riot::logger::{RiotLogSeverityLevel, RiotLogger, RiotLogTags};
use crate::core::detours;
use core::ffi;
use vsprintf::vsprintf;
use winapi::shared::minwindef::LPVOID;
use std::ffi::CString;

pub(crate) fn initialize() -> Result<(), fern::InitError> {
    let _ = fs::create_dir_all("sidonia/logs");

    log_panics::init();

    let time_now = chrono::Local::now();
    let log_file_path = format!("sidonia/logs/sidonia_{}.log", time_now.format("%d-%m-%Y_%H-%M-%S"));
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                time_now.format("[%d-%m-%Y][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        .level_for("sidonia", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        //.chain(std::io::stdout())
        .chain(fern::log_file(log_file_path)?)
        // Apply globally
        .apply()?;

    print_cool_logo_i_made();

    //unsafe { detours::initialize_riot_log_message_varg_hook(riot_log_message_varg_hook); }

    Ok(())
}

//fn riot_log_message_varg_hook() {
//    unsafe {
//        log::info!("riot log");
//
//        detours::RiotLogMessageVargHook.call();
//    }
//}

fn print_cool_logo_i_made() {
    log::info!(r"   _____ _____ _____   ____  _   _ _____          ");
    log::info!(r"  / ____|_   _|  __ \ / __ \| \ | |_   _|   /\    ");
    log::info!(r" | (___   | | | |  | | |  | |  \| | | |    /  \   ");
    log::info!(r"  \___ \  | | | |  | | |  | | . ` | | |   / /\ \  ");
    log::info!(r"  ____) |_| |_| |__| | |__| | |\  |_| |_ / ____ \ ");
    log::info!(r" |_____/|_____|_____/ \____/|_| \_|_____/_/    \_\");
}