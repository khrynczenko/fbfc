use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub enum Extension {
    Cpp,
    C,
    Hpp,
    H,
}

impl AsRef<str> for Extension {
    fn as_ref(&self) -> &str {
        match self {
            Extension::Hpp => "hpp",
            Extension::H => "h",
            Extension::Cpp => "cpp",
            Extension::C => "c",
        }
    }
}

fn has_proper_extension(file: &Path, extensions: &[Extension]) -> bool {
    let mut stringified_extensions = extensions.iter().map(|ext| ext.as_ref());

    file.extension()
        .map(|file_ext| stringified_extensions.any(|ext| ext == file_ext))
        .unwrap_or(false)
}

pub fn find_source_files(path: &Path, extensions: &[Extension]) -> Vec<PathBuf> {
    let paths = fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .map(|entry| entry.path());
    let files = paths.filter(|path| path.is_file());
    let sources = files.filter(|path| has_proper_extension(path, &extensions));

    sources.collect()
}

pub fn find_source_files_recursively(path: &Path, extensions: &[Extension]) -> Vec<PathBuf> {
    let mut sources = find_source_files(path, extensions);

    let paths = fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .map(|entry| entry.path());

    let directories = paths.filter(|path| path.is_dir());

    for directory in directories {
        sources.extend(find_source_files(&directory, extensions));
    }

    sources
}
