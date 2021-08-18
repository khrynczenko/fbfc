use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::Path;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BannedFunctionCategory {
    StringCopy,
    StringConcat,
    Sprintf,
    NSprintf,
    VarArgSprintf,
    VarArgNSprintf,
    NStringCopy,
    NStringConcat,
    StringToken,
    Makepath,
    Splitpath,
    Scanf,
    NScanf,
    NumericConversion,
    Gets,
    IsBad,
    Oem,
    Allocation,
    StringLength,
    MemoryCopy,
    WindowMessaging,
}

impl AsRef<str> for BannedFunctionCategory {
    fn as_ref(&self) -> &str {
        match self {
            BannedFunctionCategory::StringCopy => "String Copy",
            BannedFunctionCategory::StringConcat => "String Concatenation",
            BannedFunctionCategory::Sprintf => "sprintf",
            BannedFunctionCategory::NSprintf => "nsprintf",
            BannedFunctionCategory::VarArgSprintf => "Variable argument sprintf",
            BannedFunctionCategory::VarArgNSprintf => "Variable argument nsprintf",
            BannedFunctionCategory::NStringCopy => "String N Copy",
            BannedFunctionCategory::NStringConcat => "String N Concat",
            BannedFunctionCategory::StringToken => "String Token",
            BannedFunctionCategory::Makepath => "Makepath",
            BannedFunctionCategory::Splitpath => "Splitpath",
            BannedFunctionCategory::Scanf => "Scanf",
            BannedFunctionCategory::NScanf => "NScanf",
            BannedFunctionCategory::NumericConversion => "Numeric Conversion",
            BannedFunctionCategory::Gets => "Gets",
            BannedFunctionCategory::IsBad => "IsBad*",
            BannedFunctionCategory::Oem => "OEM",
            BannedFunctionCategory::Allocation => "Allocation",
            BannedFunctionCategory::StringLength => "String Length",
            BannedFunctionCategory::MemoryCopy => "Memory Copy",
            BannedFunctionCategory::WindowMessaging => "Window Messaging",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BannedFunction {
    pub category: BannedFunctionCategory,
    pub name: &'static str,
    pub possible_fix: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BannedFunctionUsage {
    pub function: &'static BannedFunction,
    pub file: String,
    pub line: usize,
    pub content: String,
}

impl Display for BannedFunctionUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Category:      {:?}", self.function.category)?;
        writeln!(f, "Function name: {}", self.function.name)?;
        writeln!(f, "File:          {}", self.file)?;
        writeln!(f, "Line #:        {}", self.line)?;
        writeln!(f, "Line:          {}", self.content)?;
        writeln!(f, "Possible fix:  {}", self.function.possible_fix)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Summary {
    pub found_usages_count: usize,
    pub found_usages_count_per_category: HashMap<BannedFunctionCategory, usize>,
}

impl Display for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(
            f,
            "Found banned functions calls: {}",
            self.found_usages_count
        )?;
        for (key, value) in &self.found_usages_count_per_category {
            writeln!(f, "{:?} found issues: {}", key, value)?;
        }
        Ok(())
    }
}

pub fn analyze_source_code<I: Iterator<Item = &'static BannedFunction> + Clone>(
    corresponding_file_path: &Path,
    source_code: &str,
    banned_functions: I,
) -> Vec<BannedFunctionUsage> {
    source_code
        .lines()
        .enumerate()
        .cartesian_product(banned_functions)
        .filter(|((_, line), function)| {
            let function_name_with_paren = function.name.to_string() + "(";
            line.contains(&function_name_with_paren) && !line.trim().starts_with("//")
        })
        .map(|((i, line), function)| BannedFunctionUsage {
            function,
            file: String::from(corresponding_file_path.to_str().unwrap()),
            line: i + 1,
            content: line.to_string(),
        })
        .collect()
}

pub fn create_summary(
    found_banned_usages: &[BannedFunctionUsage],
    active_categories: &[BannedFunctionCategory],
) -> Summary {
    Summary {
        found_usages_count: found_banned_usages.len(),
        found_usages_count_per_category: active_categories
            .iter()
            .map(|cat| {
                (
                    *cat,
                    found_banned_usages
                        .iter()
                        .filter(|usage| usage.function.category == *cat)
                        .count(),
                )
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_expected_results() -> (PathBuf, String, Vec<BannedFunctionUsage>) {
        let path = Path::new("arbitrary_path");
        let source = "int main() { strcpy() }";
        let provided_banned_functions = &[BannedFunction {
            category: BannedFunctionCategory::MemoryCopy,
            name: "strcpy",
            possible_fix: "strcpy_s",
        }];

        (
            path.to_owned(),
            String::from(source),
            vec![BannedFunctionUsage {
                function: &provided_banned_functions[0],
                file: String::from(path.to_str().unwrap()),
                line: 1,
                content: String::from(source),
            }],
        )
    }

    #[test]
    fn analyzing_source_code_without_banned_functions_provided() {
        let path = Path::new("arbitrary_path");
        let source = "int main() { strcpy() }";

        let found_usages = analyze_source_code(&path, &source, [].iter());

        assert_eq!(found_usages, Vec::new());
    }

    #[test]
    fn analyzing_source_code_with_banned_functions_provided() {
        let test_data = create_expected_results();
        let path = test_data.0;
        let source = test_data.1;
        let provided_banned_functions = &[BannedFunction {
            category: BannedFunctionCategory::MemoryCopy,
            name: "strcpy",
            possible_fix: "strcpy_s",
        }];

        let found_usages = analyze_source_code(&path, &source, provided_banned_functions.iter());

        let expected_usages = test_data.2;
        assert_eq!(found_usages, expected_usages);
    }

    #[test]
    fn analyzing_empty_source_code() {
        let path = Path::new("arbitrary_path");
        let source = "";
        let provided_banned_functions = &[BannedFunction {
            category: BannedFunctionCategory::MemoryCopy,
            name: "strcpy",
            possible_fix: "strcpy_s",
        }];

        let found_usages = analyze_source_code(&path, &source, provided_banned_functions.iter());

        assert_eq!(found_usages, vec![]);
    }

    #[test]
    fn creating_summary_without_found_usages() {
        let found_banned_usages = vec![];

        let summary = create_summary(&found_banned_usages, &[BannedFunctionCategory::MemoryCopy]);

        let expected_summary = Summary {
            found_usages_count: 0,
            found_usages_count_per_category: [(BannedFunctionCategory::MemoryCopy, 0)]
                .iter()
                .cloned()
                .collect(),
        };
        assert_eq!(summary, expected_summary);
    }

    #[test]
    fn creating_summary_with_found_usages() {
        let found_banned_usages = create_expected_results().2;

        let summary = create_summary(&found_banned_usages, &[BannedFunctionCategory::MemoryCopy]);

        let expected_summary = Summary {
            found_usages_count: 1,
            found_usages_count_per_category: [(BannedFunctionCategory::MemoryCopy, 1)]
                .iter()
                .cloned()
                .collect(),
        };
        assert_eq!(summary, expected_summary);
    }
}
