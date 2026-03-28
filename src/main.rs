use my_zip::inputs;

fn main() {
    let inputs = inputs::get_inputs();

    match inputs {
        Ok(_) => {},
        Err(e) => {
            eprint!("Error: {}", e);
            std::process::exit(1);
        },
    }
}
