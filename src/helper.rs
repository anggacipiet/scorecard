

/// Returns the current executable folder
pub fn get_executable_folder() -> Result<std::path::PathBuf, Error> {
    let mut folder = current_exe()?;
    folder.pop();
    Ok(folder)
}

pub init_log() -> Result<(), Error> {
    let mut log_path = get_exec()?;
    log_path.push(LOG_PATH);
    let mut log_dir = log_path.clone();
    log_dir.pop();
    DirBuilder::new().recursive(true).create(log_dir)?;

    if !metadata(&log_path).is_ok() {
        let config = include_str!("../log.yml");
        let mut file = File::create(&log_path)?;
        file.write_all(config.as_bytes())?;
    }
    log4rs::init_file(log_path, Default::default())?;
    Ok(())
}

/// validate path input
fn validator_path(input: String) -> Result<(), String> {
    match get_path_for_existing_file(&input) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Get path for input if possible
fn get_path_for_existing_file(input: &str) -> Result<PathBuf, String> {
    let path_o = PathBuf::from(input);
    let path;
    if path_o.parent().is_some() && path_o.parent().unwrap().is_dir() {
        path = path_o;
    } else {
        let mut path_w = std::env::current_dir().unwrap();
        path_w.push(input);
        path = path_w;
    }

    if path.is_dir() {
        return Err(format!("Specified file is a directory {:?}", path));
    }

    if !path.exists() {
        return Err(format!("Specified file not existing {:?}", path));
    }

    Ok(path)
}