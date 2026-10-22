use super::{ID, SaoError};

// Iterates through the list of sinks returned by wpctl to find the ID matching the target name
pub fn id(target: &str, sinks: Vec<String>) -> Result<ID, SaoError> {
    for line in sinks {
        if line.contains(target) {
            return Ok(line.split_whitespace().next().unwrap().trim().parse()?);
        }
    }
    Err(SaoError::NoID(target.to_string()))
}

// Finds the next valid ID based on the order of the configuration list
pub fn id_from_config(sinks: &[String], list: &[String]) -> Result<ID, SaoError> {
    let mut first_sink_id: Option<ID> = None;
    let mut current_sink_found = false;

    for value in list {
        for line in sinks {
            if line.contains(value) {
                let mut split = line.split_whitespace();
                let id: ID = split.next().unwrap().parse()?;

                if current_sink_found {
                    return Ok(id);
                }
                if first_sink_id.is_none() {
                    first_sink_id = Some(id)
                }

                let _ = split.next();
                let _ = split.next();
                if split.next().is_some() {
                    current_sink_found = true
                }
            }
        }
    }

    match first_sink_id {
        None => Err(SaoError::NoValidSinkInConfig),
        Some(id) => Ok(id),
    }
}

// Checks if the wpctl executable is available on the system using the 'which' command
pub fn check_cmd() -> Result<(), SaoError> {
    if std::process::Command::new("which")
        .arg("wpctl")
        .output()?
        .status
        .success()
    {
        Ok(())
    } else {
        Err(SaoError::NoCmd)
    }
}

// Executes 'wpctl list audio sinks' and retrieves the output lines as a vector of strings
pub fn sinks() -> Result<Vec<String>, SaoError> {
    let cmd = std::process::Command::new("wpctl")
        .arg("list")
        .arg("audio")
        .arg("sinks")
        .output()?;

    let mut sinks: Vec<String> = Vec::new();
    for sink in String::from_utf8(cmd.stdout.to_vec())?.lines() {
        sinks.push(sink.to_string());
    }

    Ok(sinks)
}
