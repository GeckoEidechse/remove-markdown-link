use remove_markdown_links::remove_markdown_links;

/// Print CLI usage
fn print_usage() {
    println!("Expected exactly one argument");
    println!("Usage: remove_markdown_links [String]");
}

fn main() {
    // Call the function from the library
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => println!("{}", remove_markdown_links(&args[1])),
        _ => {
            println!("Default");
            print_usage();
        }
    };
}
