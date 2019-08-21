//! IDLE 命令行工具
//! Python 交互式开发环境

use std::env;

/// 显示版本信息
fn print_version() {
    println!(r"IDLE 3.12.0 from e:\rusty-python\compilers\python-tools (python 3.12)");
}

/// 显示帮助信息
fn print_help() {
    println!("Usage: idle [options] [file] [args]");
    println!("");
    println!("Options:");
    println!("  -h, --help            Show this help message and exit");
    println!("  -V, --version         Show IDLE version number and exit");
    println!(
        "  -e, --edit            Edit the file
  -t, --test            Run the file as a test
  -c, --command COMMAND  Run the command
  -s, --shell            Start the shell
  -r, --rpc             Start the RPC server"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 处理命令行参数
    for arg in &args[1..] {
        match arg.as_str() {
            "--version" => {
                print_version();
                return;
            }
            "--help" => {
                print_help();
                return;
            }
            "-V" => {
                print_version();
                return;
            }
            "-h" => {
                print_help();
                return;
            }
            _ => {
                // 处理其他参数
                println!("IDLE option '{}' not yet implemented", arg);
                return;
            }
        }
    }

    // 没有参数时启动 IDLE
    println!("Starting IDLE...");
    println!("IDLE is not yet implemented");
}
