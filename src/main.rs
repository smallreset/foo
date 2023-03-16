use std::path::{Path, PathBuf};
use std::fs::{self, File, read_dir};
use std::io::{Write, Error, Read, ErrorKind};

use colored::Colorize;

mod bar;
use bar::module::Foo;

fn get_files(path: &Path, pattern: String) -> Vec<PathBuf> {
    read_dir(&path).unwrap().into_iter().filter(|x| x.as_ref().unwrap().file_name().to_string_lossy().ends_with(&pattern)).map(|x| PathBuf::from(x.unwrap().path())).collect()
}

fn main() -> Result<(), Error> {
    let files = get_files(Path::new("C:/Temp"), ".rs".to_string());
    let mut buffer = [0u8; 65536];
    let mut result: Vec<String> = Vec::new();
    println!("{:?}", files);
    for file in files {
        let mut f = File::open(&file)?;
        let cnt = f.read(&mut buffer)?;
        println!("cnt={cnt}");
        println!("{:?}", &buffer[..cnt]);
        println!("{:?}", std::str::from_utf8(&buffer[..cnt]).unwrap());
        let s = std::str::from_utf8(&buffer[..cnt]).unwrap();
        let buf = fs::read(&file)?;
        let collect_lines: Vec<String> = s.split(|x| x == '\n').filter(|x| x.contains("collect()")).map(|x| x.to_string()).collect();
        if !collect_lines.is_empty() {
            result.push(format!("{:?}:\n{}", file.file_name().unwrap(), collect_lines.join("\n")));
        }
        for line in s.split(|x| x == '\n').filter(|x| x.contains("collect()")) {
            println!("{:?}", line);
        }
    }
    println!("{:?}", result);
    {
        let mut f = File::create("C:/Temp/result.txt")?;
        f.write(result.join("\n").as_bytes())?;
    }
    Ok(())
}
