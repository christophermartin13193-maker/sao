use super::{Args, Config, ID, SaoError, Step, Verbosity, get, get_default};
use super::{check_cmd, id, id_from_config, sinks};

// Executes the system command to set the new default audio sink
pub fn wpctl(v: Verbosity, id: ID) -> Result<Step, SaoError> {
    v.dbg("STEP : WPCTL");

    //
    v.dbg("Modifying Default Sink");
    std::process::Command::new("wpctl")
        .arg("set-default")
        .arg(id.to_string())
        .output()?;
    v.dbg("Done");

    Ok(Step::Exit)
}

// Resolves a raw name or configuration alias into a sink ID
pub fn target(
    v: Verbosity,
    name: String,
    sinks: Vec<String>,
    config: Option<Config>,
) -> Result<Step, SaoError> {
    v.dbg("STEP : NAME");

    //
    v.dbg("Defining Target");
    // If a configuration exists and contains the alias, we retrieve the associated value, otherwise we keep the raw name
    let target = if let Some(mut config) = config
        && let Some(target) = config.list.remove(&name)
    {
        target
    } else {
        name
    };
    v.dbg(&format!("Target = {target}"));

    //
    v.dbg("Defining ID");
    let id = id(&target, sinks)?;
    v.dbg(&format!("ID = {id}"));

    Ok(Step::Wpctl { v, id })
}

// Determines the next sink in the configured list (--next)
pub fn next(v: Verbosity, sinks: Vec<String>, config: Config) -> Result<Step, SaoError> {
    v.dbg("STEP : NEXT");

    let list: Vec<String> = config.list.values().cloned().collect();

    //
    v.dbg("Defining Target");
    let id = id_from_config(&sinks, &list[..])?;
    v.dbg(&format!("Target = {id}"));

    Ok(Step::Wpctl { v, id })
}

// Determines the previous sink by reversing the configured list (--previous)
pub fn previous(v: Verbosity, sinks: Vec<String>, config: Config) -> Result<Step, SaoError> {
    v.dbg("STEP : PREVIOUS");

    let list: Vec<String> = config.list.values().rev().cloned().collect();

    //
    v.dbg("Defining Target");
    let id = id_from_config(&sinks, &list[..])?;
    v.dbg(&format!("Target = {id}"));

    Ok(Step::Wpctl { v, id })
}

// Initializes the application: checks dependencies, fetches sinks, and loads the appropriate configuration
pub fn init(v: Verbosity, args: Args) -> Result<Step, SaoError> {
    v.dbg("STEP : INIT");

    //
    v.dbg("Check Command");
    check_cmd()?;

    //
    v.dbg("Get Sinks");
    let sinks = sinks()?;
    if sinks.is_empty() {
        return Err(SaoError::NoSinks);
    }

    //
    v.dbg("Get Config");
    let config: Option<Config> = match (args.path, &args.name) {
        (Some(path), _) => Some(get(path)?),
        (None, None) => Some(get_default()?),
        (None, Some(_)) => match get_default() {
            Err(_) => {
                v.dbg("No config found, which is fine as long as the user didn't chose '--next' nor '--previous'");
                None
            }
            Ok(config) => Some(config),
        },
    };

    if let Some(name) = args.name {
        Ok(Step::Target {
            v,
            name,
            sinks,
            config,
        })
    } else if args.next {
        Ok(Step::Next {
            v,
            sinks,
            config: config.expect("Error already handled if the paths were compromised"),
        })
    } else {
        Ok(Step::Previous {
            v,
            sinks,
            config: config.expect("Error already handled if the paths were compromised"),
        })
    }
}
