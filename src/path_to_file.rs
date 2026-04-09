//use crate::inputs::get_inputs;
use std::{
    fs::*,
    io::BufReader,
    path::PathBuf,
    error::Error,
};

pub fn read_file(file: Vec<PathBuf>) -> Result<Vec<(BufReader<File>, String)>, Box<dyn Error>> {

    let mut file_paths: Vec<(BufReader<File>, String)> = Vec::new();

    for files in file {
        let file_contents = File::open(&files)?;
        let buffer = BufReader::new(file_contents);

        //we get the path, unwrap it, get the file name, then convert to string lossy before converting it to string.

        let file_name = files.file_name()
            .ok_or("file name not found")?
            .to_string_lossy()
            .to_string();

        file_paths.push((buffer, file_name));
    }

    Ok(file_paths)
}
