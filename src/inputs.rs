use std::{
    env,
    error::Error,
    path::PathBuf,
};

pub fn get_inputs() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let  args = env::args();

    if args.len() < 2 {
        return Err("No arguments were provided".into());
    }
    let mut paths: Vec<PathBuf> = Vec::new();

    for arguments in args {
        let file_path = PathBuf::from(arguments);
        let file = file_path.metadata()?;

        if file.is_file() {
            paths.push(file_path);
        } else {
            return Err("No such file or directory".into());
        }
    }

    Ok(paths)
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
