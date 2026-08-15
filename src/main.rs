use clap::Parser as ClapParser;
use tl_analyser::analyse;
use tl_parser::Parser;

#[derive(ClapParser)]
struct Args {
    #[arg(long, group = "execution")]
    vm: bool,

    #[arg(long, group = "execution")]
    eval: bool,

    #[arg(long)]
    debug: bool,
}

fn run_source(source: &str, args: &Args) {
    let program = match Parser::new(source).parse() {
        Ok(p) => p,
        Err(e) => { eprintln!("[parse error] {e:?}"); return; }
    };

    let errors = analyse(&program);
    if !errors.is_empty() {
        for e in &errors { eprintln!("[analysis error] {e:?}"); }
        return;
    }

    if args.eval {
        todo!("ast eval")
    } else {
        tl_vm::run(&program, tl_vm::VMOptions { debug: args.debug });
    }
}

fn main() {
    let args = Args::parse();

    loop {
        let mut input = String::new();
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        let source = input.trim();
        if source == "exit" || source.is_empty() { break; }

        run_source(source, &args);
    }
}