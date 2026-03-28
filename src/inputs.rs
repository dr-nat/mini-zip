use std::{
    env,
    error::Error,
    path::PathBuf,
};

pub fn get_inputs() -> Result<PathBuf, Box<dyn Error>> {
    let  args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("No arguments were provided".into());
    }

    let first_arg = args.get(1).ok_or("File path not found")?;

    let file_path = PathBuf::from(first_arg);

    let file = file_path.metadata()?;

    if file.is_file() {
        return Ok(file_path);
    } else {
        return Err("No such file or directory".into());
    }
}

#[cfg(test)]
mod tests{
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_args_return_value() {
        let args = get_inputs().unwrap();
        assert_eq!(args, PathBuf::from("home/natty/Desktop/Resume.pdf"));
    } 
}
