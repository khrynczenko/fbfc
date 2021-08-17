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

#[derive(Clone, Debug)]
pub struct BannedFunctionUsage<'a> {
    pub function: &'static BannedFunction,
    pub file: String,
    pub line: usize,
    pub content: String,
    pub possible_fix: &'a str,
}

impl<'a> Display for BannedFunctionUsage<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Category:      {:?}", self.function.category)?;
        writeln!(f, "Function name: {}", self.function.name)?;
        writeln!(f, "File:          {}", self.file)?;
        writeln!(f, "Line #:        {}", self.line)?;
        writeln!(f, "Line:          {}", self.content)?;
        writeln!(f, "Possible fix:  {}", self.function.possible_fix)
    }
}

#[derive(Clone, Debug)]
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
) -> Vec<BannedFunctionUsage<'static>> {
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
            possible_fix: function.possible_fix,
        })
        .collect()
}

pub fn create_summary(
    found_banned_usages: &[BannedFunctionUsage<'static>],
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
