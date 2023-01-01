use syslog::{Facility, Formatter3164,};
use sysinfo::{get_current_pid, PidExt};
use std::error::Error;

fn get_process_name() -> String {
    let this_process = std::env::current_exe().unwrap();
    let this_file = this_process.file_name().unwrap();
    
    String::from(this_file.to_str().unwrap())
}

enum Severity {
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Info,
    Debug,
}

fn log(message: &str, facility: Facility, severity: Severity) -> Result<(), Box<dyn Error>> {
    let this_pid = get_current_pid().unwrap();
    let formatter = Formatter3164 {
        facility,
        hostname: None,
        process: get_process_name(),
        pid: this_pid.as_u32(),
    };

    let mut writer = syslog::unix(formatter)?;
    writer.err(message)?;

    match severity {
        Severity::Emergency => writer.emerg(message)?,
        Severity::Alert => writer.alert(message)?,
        Severity::Critical => writer.crit(message)?,
        Severity::Error => writer.err(message)?,
        Severity::Warning => writer.warning(message)?,
        Severity::Notice => writer.notice(message)?,
        Severity::Info => writer.info(message)?,
        Severity::Debug => writer.debug(message)?,
    }

    Ok(())
}

fn main() {
    log("This is a log message", Facility::LOG_USER, Severity::Info).unwrap();
}
