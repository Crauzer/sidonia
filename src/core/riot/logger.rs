use crate::core::riot::mutex::RiotMutex;
use crate::core::msvc::vector::StdVector;
use winapi::ctypes::c_char;
use winapi::shared::minwindef::LPCVOID;

pub type RiotLogTags = u64;

#[repr(u32)]
#[derive(Debug)]
pub enum RiotLogSeverityLevel {
    Ok = 0,
    Warning = 1,
    Error = 2,
    Always = 3
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
    GetVersion: extern fn() -> u32,
    SetMinimumSeverityToBeLogged: extern fn(*mut IRiotLogTarget, RiotLogSeverityLevel),
    GetMinimumSeverityToBeLogged: extern fn(*const IRiotLogTarget) -> RiotLogSeverityLevel,
    SetTags: extern fn(*mut IRiotLogTarget, RiotLogTags, RiotLogTags),
    GetRequiredTags: extern fn(*const IRiotLogTarget) -> RiotLogTags,
    GetExcludedTags: extern fn(*const IRiotLogTarget) -> RiotLogTags,
    GetEmbellishmentFormat: extern fn(*const IRiotLogTarget, LPCVOID /*TODO: RiotLogEntry*/ ) -> *const c_char,
    Enable: extern fn(*mut IRiotLogTarget),
    Disable: extern fn(*mut IRiotLogTarget),
    IsEnabled: extern fn(*const IRiotLogTarget) -> bool,
    AppendToLog: extern fn(*mut IRiotLogTarget, LPCVOID /*TODO: RiotLogEntry*/)
}