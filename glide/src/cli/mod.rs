use clap::Parser;

#[derive(Parser,Debug)]
#[command(name="glide",version)]
pub struct Cli {

  /// project dir
  #[arg(short, long)]
  pub project_root : String
}
