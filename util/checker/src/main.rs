mod cmd;

use std::process;

fn main() {
    match cmd::check_stubs_compile() {
        Ok(exit_code) => process::exit(exit_code),
        Err(error) => {
            println!("An error occured during stub compile check:\n{}", error);

            process::exit(1);
        }
    }
}
