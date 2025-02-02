use std::str::FromStr;

mod lexer;

pub struct SourceFile(std::path::PathBuf);

fn read_env() -> SourceFile {
    use std::path::PathBuf;
    use std::{env::args, process::exit};

    let argv = args().collect::<Vec<_>>();
    let argc = argv.len();

    if argc != 2 {
        eprintln!("Expected only one argument");
        exit(1);
    }

    let path = match PathBuf::from_str(argv.get(1).unwrap()) {
        Ok(path) => path,
        Err(e) => {
            eprintln!(
                "Could not parse given param '{}' to a path due to: {e}",
                argv.get(1).unwrap()
            );
            exit(1)
        }
    };

    SourceFile(path)
}

fn main() {
    let source = read_env();

    let tokens = match lexer::scan(source) {
        Ok(tokens) => tokens,
        Err(parser_error) => panic!("{parser_error}"),
    };

    // Ast ?

    // Runner ?
}
