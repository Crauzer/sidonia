use crate::core::{msvc::vector::StdVector, riot::mutex::RiotMutex};
use winapi::{ctypes::c_char, shared::minwindef::LPCVOID};

pub type RiotLogTags = u64;

#[repr(u32)]
#[derive(Debug)]
pub enum RiotLogSeverityLevel {
    Ok = 0,
    Warning = 1,
    Error = 2,
    Always = 3,
}

#[repr(C)]
pub struct RiotLogger {
    logger_mutex: RiotMutex,
    log_target_pointers: StdVector<IRiotLogTarget>, // std::vector<IRiotLogTarget>
    break_on_error: bool,
    is_suspended: bool,
    min_severity_to_log: RiotLogSeverityLevel,
    excluded_tags: RiotLogTags,
    process_file_nane: [u8; 1024],
    process_name: *const u8,
    // TODO: Riot::Logger::StdErrLogTarget stdErrLog;
    // TODO: Riot::Logger::StdOutLogTarget stdOutLog;
    // TODO: Riot::Instrument::Metrics::MetricValueRef warningsCounter;
    // TODO: Riot::Instrument::Metrics::MetricValueRef errorsCounter;
}

/*
struct __cppobj Riot::Logger::LogTarget : IRiotLogTarget
{
  int (**_vptr$LogTarget)(void);
  volatile bool mEnabled;
  Riot::Mutex mMutex;
  Riot::Logger::LogTarget::InitParams *mParams;
};
 */

#[allow(non_snake_case)]
#[repr(C)]
pub struct IRiotLogTarget {
    GetVersion: extern "C" fn() -> u32,
    SetMinimumSeverityToBeLogged: extern "C" fn(*mut IRiotLogTarget, RiotLogSeverityLevel),
    GetMinimumSeverityToBeLogged: extern "C" fn(*const IRiotLogTarget) -> RiotLogSeverityLevel,
    SetTags: extern "C" fn(*mut IRiotLogTarget, RiotLogTags, RiotLogTags),
    GetRequiredTags: extern "C" fn(*const IRiotLogTarget) -> RiotLogTags,
    GetExcludedTags: extern "C" fn(*const IRiotLogTarget) -> RiotLogTags,
    GetEmbellishmentFormat: extern "C" fn(*const IRiotLogTarget, LPCVOID /*TODO: RiotLogEntry*/) -> *const c_char,
    Enable: extern "C" fn(*mut IRiotLogTarget),
    Disable: extern "C" fn(*mut IRiotLogTarget),
    IsEnabled: extern "C" fn(*const IRiotLogTarget) -> bool,
    AppendToLog: extern "C" fn(*mut IRiotLogTarget, LPCVOID /*TODO: RiotLogEntry*/),
}
