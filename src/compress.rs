use std::{ 
    error::Error,
    path::PathBuf,
    fs::File,
};
use zip::{
    write::SimpleFileOptions,
    CompressionMethod::Deflated,
};

use crate::path_to_file::read_file;


pub fn zip_compress(file: PathBuf) -> Result<(PathBuf, String), Box<dyn Error>> {
    let file_name = &file.file_name()
       .ok_or("unable to fetch file name")?
       .to_string_lossy()
       .to_string();

    let parent_dir = file.parent().ok_or("no parent directory found")?;
    let zip_file_name = file_name.clone() + ".zip";

    let new_path = parent_dir.join(&zip_file_name);
    let zip_file = File::create(zip_file_name)?;

    let mut zip = zip::ZipWriter::new(zip_file);

    let options = SimpleFileOptions::default()
        .compression_method(Deflated)
        .unix_permissions(0o644);
    zip.start_file(file_name.clone(), options)?;

    let original_file = read_file(file)?;

    let (mut buffer, buf_file_name) = original_file;
    let _written_file_to_zip = std::io::copy(&mut buffer, &mut zip)?;
    zip.finish()?;

    println!("File Compressed successfully");
    Ok((new_path, buf_file_name))
}
