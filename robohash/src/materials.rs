use std::path::{Path, PathBuf};

use crate::error::Error;

pub(crate) fn categories_in_set(root: &str, set: &str) -> Result<Vec<String>, Error> {
    let sets_dir = Path::new(root).join(set);
    let sets = directories_in_path(&sets_dir)?;
    Ok(sets)
}

pub(crate) fn files_in_category(
    root: &str,
    set: &str,
    category: &str,
) -> Result<Vec<String>, Error> {
    let directory = path_builder(root, set, category);
    let files = directories_in_path(&directory)?
        .iter()
        .flat_map(|dir| {
            if let Some(path) = directory.join(dir).as_path().to_str() {
                Some(String::from(path))
            } else {
                println!("cannot create directory {directory:#?}/{dir:#?}");
                None
            }
        })
        .collect::<Vec<String>>();
    Ok(files)
}

fn path_builder(sets_root: &str, set: &str, category: &str) -> PathBuf {
    Path::new(sets_root).join(set).join(category)
}

fn directories_in_path(path: &PathBuf) -> Result<Vec<String>, Error> {
    let mut directories = path
        .read_dir()?
        .into_iter()
        .filter_map(|path| match path {
            Ok(path) => match path.file_name().into_string() {
                Ok(set) => Some(set),
                Err(e) => {
                    println!("{e:#?}");
                    None
                }
            },
            Err(_) => None,
        })
        .collect::<Vec<String>>();
    directories.sort();
    Ok(directories)
}
