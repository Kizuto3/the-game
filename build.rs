use std::{env, fs, path::{Path, PathBuf}};

extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("app_icon.ico");
    res.compile().unwrap();
  }

  let src = Path::join(&env::current_dir().unwrap(), "assets");
  let out_dir = if !cfg!(debug_assertions) {
    Path::join(&env::current_dir().unwrap(), "target").join("release")
  }
  else {
    Path::join(&env::current_dir().unwrap(), "target").join("debug")
  };

  let dest = Path::join(Path::new(&out_dir), Path::new("assets"));
  copy_assets(&src, &dest);
}

fn copy_assets(source_dir: &PathBuf, output_dir: &PathBuf) {
  fs::create_dir_all(&output_dir).expect(&format!("error while creating directory {:?}", output_dir));

  let dir_info = fs::read_dir(source_dir).unwrap();

  for dir_entry in dir_info {
    match dir_entry {
        Ok(dir_entry) => {
          if dir_entry.file_type().unwrap().is_dir() {
            let output_path = Path::new(&output_dir).join(dir_entry.path().components().last().unwrap().as_os_str());

            copy_assets(&dir_entry.path(), &output_path);

            continue;
          }

          if dir_entry.file_type().unwrap().is_file() {
            let output_file = Path::new(&output_dir).join(dir_entry.path().components().last().unwrap().as_os_str());

            fs::copy(dir_entry.path(), &output_file).expect(&format!("error while copying file {:?} to {:?}", dir_entry.path(), output_file));
          }
        },
        Err(_) => continue
    }
  }
}