#[derive(clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    Solve(SolveCommandArgs),
}

#[derive(Debug, clap::Args)]
pub struct SolveCommandArgs {
    /// The DIMACS input file to be solved
    pub problem_file: String,
}
