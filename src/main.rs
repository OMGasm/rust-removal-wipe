use std::{
    env::args,
    fs::{read_dir, File},
    io::{self, Write},
    iter::repeat,
    path::PathBuf,
    time::Instant,
};
use rand::prelude::*;

fn recurse_dir(dir: impl Into<PathBuf>) -> Result<Vec<PathBuf>, io::Error> {
    let mut files = Vec::new();
    for entry in read_dir(dir.into())?.flatten() {
        let meta = entry.metadata().expect("always metadata. no err");
        if meta.is_file() {
            files.push(entry.path());
        } else {
            let mut deeper = recurse_dir(entry.path())?;
            files.append(&mut deeper);
        }
    }
    Ok(files)
}

fn open(path: &PathBuf) -> Result<File, io::Error> {
    File::options().read(true).write(true).open(path)
}

fn verify() -> Result<bool, io::Error> {
    print!("> ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().eq("DELETE"))
}

fn main() -> Result<(), io::Error> {
    let wipedir = args().nth(1).expect("provide directory to erase");
    let path = PathBuf::from(&wipedir).canonicalize()?;
    match path.try_exists() {
        Ok(true) => {println!("found!")},
        Ok(false) => return Err(io::Error::new(io::ErrorKind::NotFound, "file not found")),
        Err(e) => return Err(e)
    }
    let files = recurse_dir(&path)?;
    let num_files = files.len();
    println!("about to delete directory {} containing {num_files} files; type DELETE to continue", path.to_string_lossy());
    if !verify()? {
        println!("doing noting; exiting");
        return Ok(())
    }
    
    let num_width = num_files.to_string().len();
    for (i, file) in files.iter().enumerate() {
        let len = file.metadata()?.len();
        println!("{:>width$}/{}\t{:>5} KiB\t{}", i + 1, num_files, len / 1024, file.to_string_lossy(), width = num_width);
        let t1 = Instant::now();
        for i in  1..=2 {
            print!("pass {i}");
            let t2 = Instant::now();
            let mut handle = open(file)?;
            let mut buf: Vec<u8> = Vec::new();
            buf.resize(len as usize, 0);
            let mut rng = rand::thread_rng();
            rng.fill_bytes(&mut buf);
            handle.write_all(&buf)?;
            handle.sync_data()?;
            println!(" in {} ms", t2.elapsed().as_millis());
        }
        print!("zero pass");
        let t2 = Instant::now();
        let mut handle = open(file)?;
        handle.write_all(&(repeat(0).take(len as usize).collect::<Vec<u8>>()))?;
        println!(" in {} ms", t2.elapsed().as_millis());
        std::fs::remove_file(file)?;
        println!("finished in {} ms", t1.elapsed().as_millis());
    }
    std::fs::remove_dir_all(path)?;
    Ok(())
}
