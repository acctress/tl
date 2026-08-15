use tl_analyser::analyse;
use tl_parser::Parser;

fn main() {
    let source = "let z: number = 2";
    println!("source: {source}");

    let program = match Parser::new(source).parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[parse error] {e:?}");
            std::process::exit(1);
        }
    };

    let errors = analyse(&program);

    if errors.is_empty() {
        println!("ok, no errors");
    } else {
        for e in &errors {
            eprintln!("[analysis error] {e:?}");
        }

        std::process::exit(1);
    }
}
