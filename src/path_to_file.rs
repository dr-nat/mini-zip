use crate::inputs::get_inputs;
use std::{
    fs::*,
    io::BufReader,
    //io::Read,
    path::PathBuf,
    error::Error,
};

pub fn read_file(file: PathBuf) -> Result<(BufReader<File>, String), Box<dyn Error>> {
    let file_contents = File::open(&file)?;

    //we get the path, unwrap it, get the file name, then convert to string lossy before converting it to string.
    let file_name = get_inputs()?.file_name()
                              .ok_or("file name not found")?
                              .to_string_lossy()
                              .to_string();
    let buffer = BufReader::new(file_contents);

    Ok((buffer, file_name))
}
