use clap::Parser;

#[derive(Parser)]
#[command(name = "{{project-name}}", about = "{{project_name}}")]
struct Cli {
    /// Name to greet
    #[arg(short, long, default_value = "world")]
    name: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello, {}!", cli.name);
}
