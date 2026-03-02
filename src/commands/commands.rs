use clap::Subcommand;
use clap_complete::Shell;

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Start server", help_expected = true)]
    Server,
    #[command(about = "Reload config file", help_expected = true)]
    Reload,
    #[command(about = "Stops server", help_expected = true)]
    Stop,
    #[command(about = "Generate auto completion", help_expected = true)]
    Completions {
        #[arg(help = "Your shell name (zsh, bash, fish, elvish, powershell)")]
        shell: Shell,
    },
}
