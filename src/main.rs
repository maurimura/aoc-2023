use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Runs the specified day challenge
    #[arg(short, long, value_name = "DAY")]
    day: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(day) = cli.day {
        match day.as_str() {
            "1" => {
                day_1::run().unwrap();
            }
            "2" => {
                day_2::run().unwrap();
            }
            _ => {
                println!("Day {} not implemented", day);
            }
        }
    }
}
