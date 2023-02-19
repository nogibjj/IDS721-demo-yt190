extern crate clap;
// 这里我们声明的是我们想要使用 Arg 和 App 模块。我们希望我们的应用程序有一个 FILE 参数，它将包含一个文件路径。Clap 可以帮助我们快速实现该功能。
// 这里使用了一种链式调用方法的方式，这是一种令人非常愉悦的方式。
use clap::{Arg, App};


use std::path::Path;
use std::process;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let matches = App::new("kt")
      .version("0.1.0")
      .author("Jérémie Veillet. jeremie@example.com")
      .about("A drop in cat replacement written in Rust")
      .arg(Arg::with_name("FILE")
            .help("File to print.")
            .empty_values(false)
        )
      .get_matches();

     if let Some(file) = matches.value_of("FILE") {
        if Path::new(&file).exists() {
            match File::open(file) {
                Ok(mut f) => {
                    let mut data = String::new();
                    f.read_to_string(&mut data).expect("[kt Error] Unable to read the  file.");
                    let stdout = std::io::stdout(); // 获取全局 stdout 对象
                    let mut handle = std::io::BufWriter::new(stdout); // 可选项：将 handle 包装在缓冲区中
                    match writeln!(handle, "{}", data) {
                        Ok(_res) => {},
                        Err(err) => {
                            eprintln!("[kt Error] Unable to display the file contents. {:?}", err);
                            process::exit(1);
                        },
                    }
                }
                Err(err) => {
                    eprintln!("[kt Error] Unable to read the file. {:?}", err);
                    process::exit(1);
                },
            }
        }
        else {
            eprintln!("[kt Error] No such file or directory.");
            process::exit(1);
        }
    }
}