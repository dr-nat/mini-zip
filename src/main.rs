use my_zip::{inputs, 
    compress::zip_compress
};


fn main() {
    let inputs = inputs::get_inputs().unwrap();

    //let path = match inputs {
        //Ok(_) => {},
        //Err(e) => {
            //eprint!("Error: {}", e);
            //std::process::exit(1);
        //},
    //};

    let compressed_file = zip_compress(inputs);

    match compressed_file {
        Ok(_) => {},
        Err(err) => {
            eprint!("Error: {}", err);
            std::process::exit(2);
        }
    }
}
