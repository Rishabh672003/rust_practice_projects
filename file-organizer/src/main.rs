use std::collections::HashMap;
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::os::unix;
use std::path::{Path, PathBuf};

fn get_map() -> HashMap<&'static OsStr, &'static str> {
    let mut type_map: HashMap<&OsStr, &str> = HashMap::new();
    let image_ext = [
        OsStr::new("png"),
        OsStr::new("jpeg"),
        OsStr::new("jpg"),
        OsStr::new("gif"),
    ];
    let audio_ext = [
        OsStr::new("mp3"),
        OsStr::new("aac"),
        OsStr::new("wav"),
        OsStr::new("flac"),
    ];
    let doc_ext = [
        OsStr::new("pdf"),
        OsStr::new("docx"),
        OsStr::new("md"),
        OsStr::new("odt"),
    ];
    for i in &image_ext {
        type_map.insert(*i, "Image");
    }
    for i in &audio_ext {
        type_map.insert(*i, "Audio");
    }
    for i in &doc_ext {
        type_map.insert(*i, "Document");
    }
    type_map
}

fn walk_dir(dir: &Path) -> io::Result<()> {
    let type_map = get_map();
    let mut audio: Vec<_> = vec![];
    let mut image: Vec<_> = vec![];
    let mut doc: Vec<_> = vec![];
    let mut others: Vec<_> = vec![];

    let all_files_with_extensions = fs::read_dir(dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|x| x.path().is_file())
        .filter(|x| x.path().extension().is_some())
        .map(|x| x.path())
        .collect::<Vec<_>>();

    for i in &all_files_with_extensions {
        let ext = i.extension().unwrap();
        if let Some(value) = type_map.get(ext) {
            match *value {
                "Audio" => audio.push(i.file_name()),
                "Image" => image.push(i.file_name()),
                "Document" => doc.push(i.file_name()),
                _ => others.push(i.file_name()),
            }
        } else {
            others.push(i.file_name());
        }
    }

    fs::create_dir("/Images")?;
    fs::create_dir("/Audio")?;
    fs::create_dir("/Audio")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // create_dir()?;
    // create_and_write_file().unwrap()
    // metadata()?;
    walk_dir(Path::new("/home/rishabh/Downloads/"))?;
    Ok(())
}
