use crate::BannedFunctionUsage;

pub fn print_banned_function_usages(usages: &[BannedFunctionUsage<'static>]) {
    for usage in usages {
        println!("{}", usage);
    }
}
