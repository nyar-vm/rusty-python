//! Python 命令行工具
//! 运行 Python 脚本

use python_tools::RustyPythonFrontend;
use std::{env, fs};

/// 显示版本信息
fn print_version() {
    println!("Python 3.12.0 (2024-12-25) [x86_64-pc-windows-msvc]");
}

/// 显示帮助信息
fn print_help() {
    println!("Usage: python [options] [-c command | -m module-name | script | -]");
    println!("Options:");
    println!("  -c command     Specify the command to execute (see next section)");
    println!("  -m module-name Run library module as a script (terminates option list)");
    println!("  -V, --version  Print the Python version number and exit");
    println!("  -h, --help     Print this help message and exit");
    println!("  -b             Issue warnings about str(bytes_instance), str(bytearray_instance)");
    println!("  -B             Don't write .pyc files on import");
    println!("  -d             Turn on debug output from the parser");
    println!("  -E             Ignore environment variables like PYTHONPATH");
    println!("  -s             Don't add user site directory to sys.path");
    println!("  -S             Don't imply 'import site' on initialization");
    println!("  -u             Force the stdout and stderr streams to be unbuffered");
    println!("  -v             Verbose mode (trace import statements)");
    println!("  -W arg         Warning control (arg is action:message:category:module:lineno)");
    println!("  --help-commands List available commands");
    println!("  --help-all     Show all help options");
    println!("  --isolated     Run Python in isolated mode");
    println!("  --check-hash-based-pycs always|default|never");
    println!("  --display-pip-version Show the version of pip and exit");
    println!("  --help-pip     Show pip help");
    println!("  --help-plugins Show information about plugins");
    println!("  --list-plugins  List available plugins");
    println!("  --no-plugins   Disable all plugins");
    println!("  --parser-error-display style");
    println!("  --plugins      Load plugins");
    println!("  --pycache-prefix path");
    println!("  --statistics   Show compilation statistics");
    println!("  --version      Show program's version number and exit");
    println!("  --warn-script-location");
    println!("  --help         Show this help message and exit");
}

/// 运行 Python 脚本
fn run_script(script_path: &str) {
    println!("Running Python script: {}", script_path);

    match fs::read_to_string(script_path) {
        Ok(content) => {
            let frontend = RustyPythonFrontend::new();
            match frontend.parse(&content) {
                Ok(ast) => {
                    println!("Script parsed successfully");
                    println!("AST: {:?}", ast);
                }
                Err(e) => {
                    println!("Error parsing script: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error reading script: {:?}", e);
        }
    }
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
            _ if arg.starts_with('-') => {
                // 处理其他参数
                println!("Warning: Option {} not yet implemented", arg);
            }
            _ => {
                // 执行脚本文件
                run_script(arg);
                return;
            }
        }
    }

    // 没有参数时显示帮助信息
    print_help();
}
