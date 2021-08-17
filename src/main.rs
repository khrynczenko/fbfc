#![deny(
    warnings,
    missing_debug_implementations,
    rust_2018_idioms,
    nonstandard_style,
    future_incompatible,
    clippy::all
)]
#![forbid(unsafe_code)]

mod analysis;
mod banned;
mod cli;
mod files;
mod print;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;

use analysis::{BannedFunction, BannedFunctionCategory, BannedFunctionUsage};
use banned::BANNED_FUNCTIONS;
use files::Extension;

pub fn analyze_source_file<I: Iterator<Item = &'static BannedFunction> + Clone>(
    file: &Path,
    banned_functions: I,
) -> Vec<BannedFunctionUsage<'static>> {
    let contents = fs::read_to_string(file).unwrap();
    analysis::analyze_source_code(file, &contents, banned_functions)
}

pub fn find_banned_usages<I: Iterator<Item = &'static BannedFunction> + Clone>(
    files: &[PathBuf],
    banned_functions: I,
) -> Vec<BannedFunctionUsage<'static>> {
    files
        .iter()
        .flat_map(|file| analyze_source_file(file, banned_functions.clone()))
        .collect()
}

fn main() {
    let cli_arguments = cli::run_cli();

    let active_extensions = [Extension::Hpp, Extension::H, Extension::Cpp, Extension::C];
    let active_categories = [
        BannedFunctionCategory::StringCopy,
        BannedFunctionCategory::StringConcat,
        BannedFunctionCategory::Sprintf,
        BannedFunctionCategory::NSprintf,
        BannedFunctionCategory::VarArgSprintf,
        BannedFunctionCategory::VarArgNSprintf,
        BannedFunctionCategory::NStringCopy,
        BannedFunctionCategory::NStringConcat,
        BannedFunctionCategory::StringToken,
        BannedFunctionCategory::Makepath,
        BannedFunctionCategory::Splitpath,
        BannedFunctionCategory::Scanf,
        BannedFunctionCategory::NScanf,
        BannedFunctionCategory::NumericConversion,
        BannedFunctionCategory::Gets,
        BannedFunctionCategory::IsBad,
        BannedFunctionCategory::Oem,
        BannedFunctionCategory::Allocation,
        BannedFunctionCategory::StringLength,
        BannedFunctionCategory::MemoryCopy,
        BannedFunctionCategory::WindowMessaging,
    ];

    let active_banned_functions = BANNED_FUNCTIONS
        .iter()
        .filter(|function| active_categories.contains(&function.category));

    let source_files = match cli_arguments.recursive_search {
        false => files::find_source_files(
            cli_arguments.searched_directory.as_ref(),
            &active_extensions,
        ),
        true => files::find_source_files_recursively(
            cli_arguments.searched_directory.as_ref(),
            &active_extensions,
        ),
    };

    let found_banned_usages = find_banned_usages(&source_files, active_banned_functions);

    let summary = analysis::create_summary(&found_banned_usages, &active_categories);

    if !cli_arguments.only_summary {
        print::print_banned_function_usages(&found_banned_usages);
    }

    print!("{}", &summary);

    if found_banned_usages.is_empty() {
        process::exit(0);
    }

    process::exit(1);
}
