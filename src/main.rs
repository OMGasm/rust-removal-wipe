use std::{env::args, io, fs::{read_dir, DirEntry}};

fn recurse_dir(dir: impl Into<String>) -> Result<Vec<DirEntry>, io::Error> {
    let mut files = Vec::new();
    for entry in read_dir(dir.into())? {
        if let Ok(entry) = entry {
            let meta = entry.metadata().expect("always metadata. no err");
            if meta.is_file() {
                files.push(entry);
            } else {
                let mut deeper = recurse_dir(entry.path().to_string_lossy())?;
                files.append(&mut deeper);
            }
        }
    }
    Ok(files)
}

fn main() -> Result<(), io::Error>{
    let wipedir = args().next().expect("provide directory to erase");
    let _dir = recurse_dir(wipedir)?;
    Ok(())
}
