mod runner;
mod scopes;

fn read_env() -> std::path::PathBuf {
    use std::{env::args, path::PathBuf, process::exit, str::FromStr};

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

    path
}

fn main() {
    let source = read_env();

    let lexer_tokens = match lexer::scan_file(source) {
        Ok(tokens) => tokens,
        Err(lexer_error) => panic!("{lexer_error}"),
    };

    let _parser_tokens = match parser::parse(&lexer_tokens) {
        Ok(tokens) => tokens,
        Err(parser_error) => panic!("{parser_error}"),
    };

    // Ast ?

    // Runner ?
}
