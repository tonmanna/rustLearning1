use std::fs::File;
use std::io::prelude::*;
use std::env;

pub fn ReadFS(msg: &str) -> std::io::Result<String> {
    let path = env::current_dir()?;
    let FullPath = format!("{}\\{}", path.display(), msg);
    println!("{}",FullPath);
    let mut file = File::open(FullPath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}",contents.to_string())
    Ok(contents.to_string())
    // Ok(FullPath.to_string())

}
