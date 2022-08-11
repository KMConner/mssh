use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser, PartialEq)]
#[clap(name = "struct", author, about, version)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, PartialEq)]
enum Commands {
    Exec {
        #[clap(multiple_occurrences = true, required = true)]
        servers: Vec<String>,
        #[clap(long, short = 'f')]
        file: Option<PathBuf>,
    },
    Cp {},
}

pub fn parse() -> Result<Cli, clap::error::Error> {
    Cli::try_parse()
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_args(args: Vec<&str>) -> Cli {
        Cli::parse_from(args)
    }

    mod exec_test {
        use super::*;

        #[test]
        fn run_on_single_server() {
            let cli = Cli::parse_from(vec!["mssh", "exec", "-f", "run.sh", "server1"]);
            assert_eq!(cli,
                       Cli {
                           command: Commands::Exec {
                               file: Some(PathBuf::from("run.sh")),
                               servers: vec![String::from("server1")],
                           }
                       });
        }

        #[test]
        fn run_on_multiple_servers() {
            let cli = Cli::parse_from(vec!["mssh", "exec", "-f", "run.sh", "server1", "server2", "server3"]);
            assert_eq!(cli,
                       Cli {
                           command: Commands::Exec {
                               file: Some(PathBuf::from("run.sh")),
                               servers: vec![String::from("server1"), String::from("server2"), String::from("server3")],
                           }
                       });
        }

        #[test]
        fn run_on_no_file() {
            let cli = Cli::parse_from(vec!["mssh", "exec", "server1", "server2", "server3"]);
            assert_eq!(cli, Cli {
                command: Commands::Exec {
                    file: None,
                    servers: vec![String::from("server1"), String::from("server2"), String::from("server3")],
                }
            });
        }

        #[test]
        fn error_on_no_servers() {
            let cli = Cli::try_parse_from(vec!["mssh", "exec", "-f", "run.sh"]);
            assert_eq!(cli.is_err(), true);
        }
    }
}
