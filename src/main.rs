use my_zip::{inputs, 
    compress::zip_compress
};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let inputs = inputs::get_inputs()?;

    let compressed_file = zip_compress(inputs);

    match compressed_file {
        Ok(_) => {},
        Err(err) => {
            eprint!("Error: {}", err);
            std::process::exit(2);
        }
    }

    Ok(())
}
