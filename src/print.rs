use crate::BannedFunctionUsage;

pub fn print_banned_function_usages(usages: &[BannedFunctionUsage]) {
    for usage in usages {
        println!("{}", usage);
    }
}
