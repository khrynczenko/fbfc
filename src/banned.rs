use crate::analysis::{BannedFunction, BannedFunctionCategory};

const STRING_COPY_POSSIBLE_FIX: &str = "Use String*1Copy or String*CopyEx or strcpy_s function.";

const STRING_CONCAT_POSSIBLE_FIX: &str = "Use String*Cat or String*CatEx or strcat_s function.";

const SPRINTF_POSSIBLE_FIX: &str = "Use String*Printf or String*PrintfEx or sprintf_s function.";

const NSPRINTF_POSSIBLE_FIX: &str =
    "Use String*Printf or String*PrintfEx _snprintf_s or _snwprintf_s function.";

const VAR_ARG_SPRINTF_POSSIBLE_FIX: &str =
    "Use String*VPrintf or String*VPrintfEx or _vstprintf_s function.";

const VAR_ARG_NSPRINTF_POSSIBLE_FIX: &str =
    "Use String*VPrintf or String*VPrintfEx or vsntprintf_s function.";

const NSTRING_COPY_POSSIBLE_FIX: &str = "Use String*CopyN or String*CopyNEx or strncpy_s function.";

const NSTRING_CONCAT_POSSIBLE_FIX: &str =
    "Use String*CopyN or String*CopyNEx or strncpy_s function.";

const STRING_TOKEN_POSSIBLE_FIX: &str = "Use strtok_s function.";

const MAKEPATH_POSSIBLE_FIX: &str = "Use _makepath_s function.";

const SPLITPATH_POSSIBLE_FIX: &str = "Use _splitpath_s function.";

const SCANF_POSSIBLE_FIX: &str = "Use sscanf_s function.";

const NSCANF_POSSIBLE_FIX: &str = "Use _snscanf_s function.";

const NUMERIC_CONVERSION_POSSIBLE_FIX: &str = "Use _itoa_s or _itow_s function.";

const GETS_POSSIBLE_FIX: &str = "Use String*Gets or gets_s function.";

const IS_BAD_POSSIBLE_FIX: &str = "No corresponding replacements. Avoid using these functions.";

const OEM_POSSIBLE_FIX: &str = "Use WideCharToMultiByte function.";

const ALLOCATION_POSSIBLE_FIX: &str = "Use SafeAllocA function.";

const STRING_LENGTH_POSSIBLE_FIX: &str = "Use String*Length or strnlen_s function.";

const MEMORY_COPY_POSSIBLE_FIX: &str = "Use memcpy_s or wmemcpy_s function.";

const WINDOW_MESSAGING_POSSIBLE_FIX: &str = "Use ChangeWindowMessageFilterEx function.";

pub const BANNED_FUNCTIONS: [BannedFunction; 199] = [
    BannedFunction {
        name: "strcpy",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strcpyA,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strcpyW",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wcscpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_tcscpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbscpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCpyA,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCpyW,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcpyA,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcpyW,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_tccpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbccpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_ftcscpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strncpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wcsncpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_tcsncpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbsncpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbsnbcpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCpyN,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCpyNA,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCpyNW,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCpy,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strcpynA,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCpyA,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCpyW,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcpyn,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcpynA,",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcpyn",
        category: BannedFunctionCategory::StringCopy,
        possible_fix: STRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strcat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strcatA",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strcatW",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wcscat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_tcscat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbscat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatA",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatW",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcatA",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcatW",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatBuff",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatBuffA",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatBuffW",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatChainW",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_tccat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbccat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_ftcscat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strncat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wcsncat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_tcsncat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbsncat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbsnbcat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatN",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatNA",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatNW",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCatA",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCatW",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrncat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcatnA",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcatnW",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcat",
        category: BannedFunctionCategory::StringConcat,
        possible_fix: STRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "sprintfW",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "sprintfA",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wsprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wsprintfW",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wsprintfA",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "sprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "swprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_stprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvsprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvsprintfA",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvsprintfW",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "vsprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_vstprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "vswprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wnsprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wnsprintfA",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wnsprintfW",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_snwprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "snprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "sntprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_vsnprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "vsnprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_vsnwprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_vsntprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvnsprintf",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvnsprintfA",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvnsprintfW",
        category: BannedFunctionCategory::Sprintf,
        possible_fix: SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_snwprintf",
        category: BannedFunctionCategory::NSprintf,
        possible_fix: NSPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_snprintf",
        category: BannedFunctionCategory::NSprintf,
        possible_fix: NSPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_sntprintf",
        category: BannedFunctionCategory::NSprintf,
        possible_fix: NSPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "nsprintf",
        category: BannedFunctionCategory::NSprintf,
        possible_fix: NSPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvsprintf",
        category: BannedFunctionCategory::VarArgSprintf,
        possible_fix: VAR_ARG_SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvsprintfA",
        category: BannedFunctionCategory::VarArgSprintf,
        possible_fix: VAR_ARG_SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvsprintfW",
        category: BannedFunctionCategory::VarArgSprintf,
        possible_fix: VAR_ARG_SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "vsprintf",
        category: BannedFunctionCategory::VarArgSprintf,
        possible_fix: VAR_ARG_SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_vstprintf",
        category: BannedFunctionCategory::VarArgSprintf,
        possible_fix: VAR_ARG_SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "vswprintf",
        category: BannedFunctionCategory::VarArgSprintf,
        possible_fix: VAR_ARG_SPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_vsnprintf",
        category: BannedFunctionCategory::VarArgNSprintf,
        possible_fix: VAR_ARG_NSPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_vsnwprintf",
        category: BannedFunctionCategory::VarArgNSprintf,
        possible_fix: VAR_ARG_NSPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_vsntprintf",
        category: BannedFunctionCategory::VarArgNSprintf,
        possible_fix: VAR_ARG_NSPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvnsprintf",
        category: BannedFunctionCategory::VarArgNSprintf,
        possible_fix: VAR_ARG_NSPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvnsprintfA",
        category: BannedFunctionCategory::VarArgNSprintf,
        possible_fix: VAR_ARG_NSPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wvnsprintf",
        category: BannedFunctionCategory::VarArgNSprintf,
        possible_fix: VAR_ARG_NSPRINTF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strncpy",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wcsncpy",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_tcsncpy",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbsncpy",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbsnbcpy",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCpyN",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCpyNA",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCpyNW",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCpy",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strcpynA",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCpyA",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCpyW",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcpyn",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcpynA",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcpynW",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_fstrncpy",
        category: BannedFunctionCategory::NStringCopy,
        possible_fix: NSTRING_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strncat",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wcsncat",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_tcsncat",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbsncat",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbsnbcat",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatN",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatNA",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrCatNW",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCat",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCatA",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrNCatW",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrncat",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcatnA",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcatnW",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrcatn",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_fstrncat",
        category: BannedFunctionCategory::NStringConcat,
        possible_fix: NSTRING_CONCAT_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strtok",
        category: BannedFunctionCategory::StringToken,
        possible_fix: STRING_TOKEN_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "tcstok",
        category: BannedFunctionCategory::StringToken,
        possible_fix: STRING_TOKEN_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wcstok",
        category: BannedFunctionCategory::StringToken,
        possible_fix: STRING_TOKEN_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbstok",
        category: BannedFunctionCategory::StringToken,
        possible_fix: STRING_TOKEN_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "makepath",
        category: BannedFunctionCategory::Makepath,
        possible_fix: MAKEPATH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_tmakepath",
        category: BannedFunctionCategory::Makepath,
        possible_fix: MAKEPATH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_makepath",
        category: BannedFunctionCategory::Makepath,
        possible_fix: MAKEPATH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_wmakepath",
        category: BannedFunctionCategory::Makepath,
        possible_fix: MAKEPATH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_splitpath",
        category: BannedFunctionCategory::Splitpath,
        possible_fix: SPLITPATH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_tsplitpath",
        category: BannedFunctionCategory::Splitpath,
        possible_fix: SPLITPATH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_wsplitpath",
        category: BannedFunctionCategory::Splitpath,
        possible_fix: SPLITPATH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "scanf",
        category: BannedFunctionCategory::Scanf,
        possible_fix: SCANF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wscanf",
        category: BannedFunctionCategory::Scanf,
        possible_fix: SCANF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_tscanf",
        category: BannedFunctionCategory::Scanf,
        possible_fix: SCANF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "sscanf",
        category: BannedFunctionCategory::Scanf,
        possible_fix: SCANF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "swscanf",
        category: BannedFunctionCategory::Scanf,
        possible_fix: SCANF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_stscan",
        category: BannedFunctionCategory::Scanf,
        possible_fix: SCANF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "snscanf",
        category: BannedFunctionCategory::NScanf,
        possible_fix: NSCANF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "snwscanf",
        category: BannedFunctionCategory::NScanf,
        possible_fix: NSCANF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_sntscanf",
        category: BannedFunctionCategory::NScanf,
        possible_fix: NSCANF_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "itoa",
        category: BannedFunctionCategory::NumericConversion,
        possible_fix: NUMERIC_CONVERSION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "itow",
        category: BannedFunctionCategory::NumericConversion,
        possible_fix: NUMERIC_CONVERSION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_i64toa",
        category: BannedFunctionCategory::NumericConversion,
        possible_fix: NUMERIC_CONVERSION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_i64tow",
        category: BannedFunctionCategory::NumericConversion,
        possible_fix: NUMERIC_CONVERSION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_ui64toa",
        category: BannedFunctionCategory::NumericConversion,
        possible_fix: NUMERIC_CONVERSION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_ui64tot",
        category: BannedFunctionCategory::NumericConversion,
        possible_fix: NUMERIC_CONVERSION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_ui64tow",
        category: BannedFunctionCategory::NumericConversion,
        possible_fix: NUMERIC_CONVERSION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_ultoa",
        category: BannedFunctionCategory::NumericConversion,
        possible_fix: NUMERIC_CONVERSION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_ultot",
        category: BannedFunctionCategory::NumericConversion,
        possible_fix: NUMERIC_CONVERSION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_ultow",
        category: BannedFunctionCategory::NumericConversion,
        possible_fix: NUMERIC_CONVERSION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "gets",
        category: BannedFunctionCategory::Gets,
        possible_fix: GETS_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_getts",
        category: BannedFunctionCategory::Gets,
        possible_fix: GETS_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_gettws",
        category: BannedFunctionCategory::Gets,
        possible_fix: GETS_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "IsBadWritePtr",
        category: BannedFunctionCategory::IsBad,
        possible_fix: IS_BAD_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "IsBadHugeWritePtr",
        category: BannedFunctionCategory::IsBad,
        possible_fix: IS_BAD_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "IsBadReadPtr",
        category: BannedFunctionCategory::IsBad,
        possible_fix: IS_BAD_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "IsBadHugeReadPtr",
        category: BannedFunctionCategory::IsBad,
        possible_fix: IS_BAD_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "IsBadCodePtr",
        category: BannedFunctionCategory::IsBad,
        possible_fix: IS_BAD_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "IsBadStringPtr",
        category: BannedFunctionCategory::IsBad,
        possible_fix: IS_BAD_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "CharToOem",
        category: BannedFunctionCategory::Oem,
        possible_fix: OEM_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "CharToOemA",
        category: BannedFunctionCategory::Oem,
        possible_fix: OEM_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "CharToOemW",
        category: BannedFunctionCategory::Oem,
        possible_fix: OEM_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "OemToChar",
        category: BannedFunctionCategory::Oem,
        possible_fix: OEM_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "OemToCharA",
        category: BannedFunctionCategory::Oem,
        possible_fix: OEM_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "OemToCharW",
        category: BannedFunctionCategory::Oem,
        possible_fix: OEM_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "CharToOemBuffA",
        category: BannedFunctionCategory::Oem,
        possible_fix: OEM_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "CharToOemBuffW",
        category: BannedFunctionCategory::Oem,
        possible_fix: OEM_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "alloca",
        category: BannedFunctionCategory::Allocation,
        possible_fix: ALLOCATION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_alloca",
        category: BannedFunctionCategory::Allocation,
        possible_fix: ALLOCATION_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "strlen",
        category: BannedFunctionCategory::StringLength,
        possible_fix: STRING_LENGTH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wcslen",
        category: BannedFunctionCategory::StringLength,
        possible_fix: STRING_LENGTH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbslen",
        category: BannedFunctionCategory::StringLength,
        possible_fix: STRING_LENGTH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "_mbstrlen",
        category: BannedFunctionCategory::StringLength,
        possible_fix: STRING_LENGTH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "StrLen",
        category: BannedFunctionCategory::StringLength,
        possible_fix: STRING_LENGTH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "lstrle",
        category: BannedFunctionCategory::StringLength,
        possible_fix: STRING_LENGTH_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "memcpy",
        category: BannedFunctionCategory::MemoryCopy,
        possible_fix: MEMORY_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "RtlCopyMemory",
        category: BannedFunctionCategory::MemoryCopy,
        possible_fix: MEMORY_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "CopyMemory",
        category: BannedFunctionCategory::MemoryCopy,
        possible_fix: MEMORY_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "wmemcp",
        category: BannedFunctionCategory::MemoryCopy,
        possible_fix: MEMORY_COPY_POSSIBLE_FIX,
    },
    BannedFunction {
        name: "ChangeWindowMessageFilter",
        category: BannedFunctionCategory::WindowMessaging,
        possible_fix: WINDOW_MESSAGING_POSSIBLE_FIX,
    },
];
