fn print_usage(prog: &str) {
    eprintln!("Usage: {} <command>\n\nCommands:\n  list [DIR]    list grouped .pkg.* files\n  help          show this message", prog);
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }
    match args[1].as_str() {
        "list" => println!("(list command coming soon)"),
        "help" | _ => print_usage(&args[0]),
    }
}
