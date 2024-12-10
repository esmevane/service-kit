use clap::Parser;
use duct::cmd;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
enum Command {
    WebBuild,
}

impl Command {
    fn run(&self) {
        match self {
            Command::WebBuild => {
                cmd!(
                    "wasm-pack",
                    "build",
                    "--target",
                    "web",
                    "--out-dir",
                    "../service-kit-core/dist/wasm",
                    "service-kit-web"
                )
                .run()
                .expect("Failed to build web library");
            }
        }
    }
}

fn main() {
    Cli::parse().command.run();
}
