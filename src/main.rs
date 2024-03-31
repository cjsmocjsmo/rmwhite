use std::env;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <apath>", args[0]);
        return;
    }
    let apath = &args[1];
    walk_dir(apath);
}

fn walk_dir(apath: &str) {
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            // replace whitespace with _
            let new_fname = fname.replace(",", "").replace(" ", "_").to_lowercase();
            // rename file
            std::fs::rename(&fname, &new_fname).unwrap();
            println!("renamed {} -> {}", fname, new_fname);
        }
    }
}