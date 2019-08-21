//! python-config 命令行工具
//! Python 配置工具

use std::env;

/// 显示版本信息
fn print_version() {
    println!(r"python-config 3.12.0 from e:\rusty-python\compilers\python-tools (python 3.12)");
}

/// 显示帮助信息
fn print_help() {
    println!(
        "Usage: python-config [--prefix|--exec-prefix|--includes|--libs|--cflags|--ldflags|--extension-suffix|--help|--version]"
    );
    println!("");
    println!("Options:");
    println!("  --prefix            Print prefix directory");
    println!("  --exec-prefix       Print exec-prefix directory");
    println!("  --includes          Print include directories");
    println!("  --libs              Print library directories and libraries");
    println!("  --cflags            Print C compiler flags");
    println!("  --ldflags           Print linker flags");
    println!("  --extension-suffix  Print extension suffix");
    println!("  --help              Show this help message and exit");
    println!("  --version           Show version information and exit");
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
            "--prefix" => {
                println!("e:\\rusty-python");
                return;
            }
            "--exec-prefix" => {
                println!("e:\\rusty-python");
                return;
            }
            "--includes" => {
                println!(r"-Ie:\rusty-python\compilers\python\src");
                return;
            }
            "--libs" => {
                println!(r"-Le:\rusty-python\target\debug -lpython");
                return;
            }
            "--cflags" => {
                println!(r"-Ie:\rusty-python\compilers\python\src");
                return;
            }
            "--ldflags" => {
                println!(r"-Le:\rusty-python\target\debug -lpython");
                return;
            }
            "--extension-suffix" => {
                println!(".pyd");
                return;
            }
            _ => {
                // 处理其他参数
                println!("python-config option '{}' not yet implemented", arg);
                return;
            }
        }
    }

    // 没有参数时显示帮助信息
    print_help();
}
