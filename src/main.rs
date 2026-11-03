use anyhow::Result;
use clap::Parser;
use dpkg::dpkg;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    auto: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = dpkg::Config { auto_start: cli.auto, ..Default::default() };
    let mut component = dpkg::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-dpkg operational");
    Ok(())
}
