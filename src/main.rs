use clap::Command;

fn cli_main() -> Command {
    Command::new("flameforge").about("Next generation package manager")
}

fn main() {
    match cli_main().get_matches().subcommand() {
        _ => println!("我们应该实现一个真正的 CLI，嗯？")
    }
}
