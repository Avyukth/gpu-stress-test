use clap::{Parser};
use gpu_stress_test::{cpu_load_test, gpu_load_test, tgpu_load_test};
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Cpu {},
    Gpu {},
    Tgpu {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Cpu {}) => {
            println!("Running CPU Stress Test");
            cpu_load_test();
        }
        Some(Commands::Gpu {}) => {
            println!("Running GPU Stress Test");
            gpu_load_test();
        }
        Some(Commands::Tgpu {}) => {
            println!("Running TGPU Stress Test");
            tgpu_load_test();
        }
        None => {
            println!("No command given");
        }
    }
}
