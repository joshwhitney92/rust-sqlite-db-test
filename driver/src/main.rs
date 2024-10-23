use json_parser::*;
fn main() {
    let args = json_parser::io::parse_args(true);

    // echo back the args
    args.iter().for_each(|arg| print!("{} ", arg.trim()));
}
