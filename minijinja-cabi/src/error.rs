use std::cell::RefCell;
use std::error::Error as _;
use std::ffi::{c_char, CString};
use std::fmt::Write;
use std::ptr;

use minijinja::{Error, ErrorKind};

thread_local! {
    pub static LAST_ERROR: RefCell<Option<Error>> = const { RefCell::new(None) };
}

/// Returns `true` if there is currently an error.
#[no_mangle]
pub extern "C" fn mj_err_is_set() -> bool {
    LAST_ERROR.with_borrow(|x| x.is_some())
}

/// Clears the current error.
#[no_mangle]
pub extern "C" fn mj_err_clear() {
    LAST_ERROR.with_borrow_mut(|x| *x = None);
}

/// Prints the error to stderr.
#[no_mangle]
pub extern "C" fn mj_err_print() -> bool {
    LAST_ERROR.with_borrow(|x| {
        if let Some(err) = x {
            eprintln!("error: {err}");
            if err.name().is_some() {
                eprintln!("{}", err.display_debug_info());
            }
            let mut source_opt = err.source();
            while let Some(source) = source_opt {
                eprintln!();
                eprintln!("caused by: {source}");
                if let Some(source) = source.downcast_ref::<Error>() {
                    if source.name().is_some() {
                        eprintln!("{}", source.display_debug_info());
                    }
                }
                source_opt = source.source();
            }
            true
        } else {
            false
        }
    })
}

/// Returns the error's debug info if there is an error.
///
/// The value must be freed with `mj_str_free`.
#[no_mangle]
pub unsafe extern "C" fn mj_err_get_debug_info() -> *mut c_char {
    LAST_ERROR
        .with_borrow(|x| {
            x.as_ref()
                .and_then(|x| {
                    let mut info = String::new();
                    if x.name().is_some() {
                        let err_info = x.display_debug_info().to_string();
                        if !err_info.is_empty() {
                            writeln!(info, "{err_info}").unwrap();
                        }
                    }
                    let mut source_opt = x.source();
                    while let Some(source) = source_opt {
                        writeln!(info, "\ncaused by: {source}").unwrap();
                        if let Some(source) = source.downcast_ref::<Error>() {
                            if source.name().is_some() {
                                let src_info = x.display_debug_info().to_string();
                                if !src_info.is_empty() {
                                    writeln!(info, "{src_info}").unwrap();
                                }
                            }
                        }
                        source_opt = source.source();
                    }
                    info.into()
                })
                .and_then(|info| CString::new(info).ok())
                .map(|cstr| cstr.into_raw())
        })
        .unwrap_or(ptr::null_mut())
}

/// Returns the error's description if there is an error.
///
/// The value must be freed with `mj_str_free`.
#[no_mangle]
pub unsafe extern "C" fn mj_err_get_detail() -> *mut c_char {
    LAST_ERROR
        .with_borrow(|x| {
            x.as_ref()
                .and_then(|x| x.detail())
                .and_then(|detail| CString::new(detail).ok())
                .map(|cstr| cstr.into_raw())
        })
        .unwrap_or(ptr::null_mut())
}

/// Returns the error's current template.
///
/// The value must be freed with `mj_str_free`.
#[no_mangle]
pub unsafe extern "C" fn mj_err_get_template_name() -> *mut c_char {
    LAST_ERROR
        .with_borrow(|x| {
            x.as_ref()
                .and_then(|x| x.name())
                .and_then(|name| CString::new(name).ok())
                .map(|cstr| cstr.into_raw())
        })
        .unwrap_or(ptr::null_mut())
}

/// Returns the error's current line.
#[no_mangle]
pub unsafe extern "C" fn mj_err_get_line() -> u32 {
    LAST_ERROR
        .with_borrow(|x| x.as_ref().and_then(|x| x.line()))
        .unwrap_or(0) as _
}

/// The kind of error that occurred.
#[repr(C)]
pub enum mj_err_kind {
    MJ_ERR_KIND_NON_PRIMITIVE,
    MJ_ERR_KIND_NON_KEY,
    MJ_ERR_KIND_INVALID_OPERATION,
    MJ_ERR_KIND_SYNTAX_ERROR,
    MJ_ERR_KIND_TEMPLATE_NOT_FOUND,
    MJ_ERR_KIND_TOO_MANY_ARGUMENTS,
    MJ_ERR_KIND_MISSING_ARGUMENT,
    MJ_ERR_KIND_UNKNOWN_FILTER,
    MJ_ERR_KIND_UNKNOWN_FUNCTION,
    MJ_ERR_KIND_UNKNOWN_TEST,
    MJ_ERR_KIND_UNKNOWN_METHOD,
    MJ_ERR_KIND_BAD_ESCAPE,
    MJ_ERR_KIND_UNDEFINED_ERROR,
    MJ_ERROR_KIND_BAD_SERIALIZTION,
    MJ_ERR_KIND_BAD_INCLUDE,
    MJ_ERR_KIND_EVAL_BLOCK,
    MJ_ERR_KIND_CANNOT_UNPACK,
    MJ_ERR_KIND_WRITE_FAILURE,
    MJ_ERR_KIND_UNKNOWN,
}

impl TryFrom<ErrorKind> for mj_err_kind {
    type Error = ();

    fn try_from(value: ErrorKind) -> Result<Self, Self::Error> {
        Ok(match value {
            ErrorKind::NonPrimitive => mj_err_kind::MJ_ERR_KIND_NON_PRIMITIVE,
            ErrorKind::NonKey => mj_err_kind::MJ_ERR_KIND_NON_KEY,
            ErrorKind::InvalidOperation => mj_err_kind::MJ_ERR_KIND_INVALID_OPERATION,
            ErrorKind::SyntaxError => mj_err_kind::MJ_ERR_KIND_SYNTAX_ERROR,
            ErrorKind::TemplateNotFound => mj_err_kind::MJ_ERR_KIND_TEMPLATE_NOT_FOUND,
            ErrorKind::TooManyArguments => mj_err_kind::MJ_ERR_KIND_TOO_MANY_ARGUMENTS,
            ErrorKind::MissingArgument => mj_err_kind::MJ_ERR_KIND_MISSING_ARGUMENT,
            ErrorKind::UnknownFilter => mj_err_kind::MJ_ERR_KIND_UNKNOWN_FILTER,
            ErrorKind::UnknownTest => mj_err_kind::MJ_ERR_KIND_UNKNOWN_TEST,
            ErrorKind::UnknownFunction => mj_err_kind::MJ_ERR_KIND_UNKNOWN_FUNCTION,
            ErrorKind::UnknownMethod => mj_err_kind::MJ_ERR_KIND_UNKNOWN_METHOD,
            ErrorKind::BadEscape => mj_err_kind::MJ_ERR_KIND_BAD_ESCAPE,
            ErrorKind::UndefinedError => mj_err_kind::MJ_ERR_KIND_UNDEFINED_ERROR,
            ErrorKind::BadSerialization => mj_err_kind::MJ_ERROR_KIND_BAD_SERIALIZTION,
            ErrorKind::BadInclude => mj_err_kind::MJ_ERR_KIND_BAD_INCLUDE,
            ErrorKind::EvalBlock => mj_err_kind::MJ_ERR_KIND_EVAL_BLOCK,
            ErrorKind::CannotUnpack => mj_err_kind::MJ_ERR_KIND_CANNOT_UNPACK,
            ErrorKind::WriteFailure => mj_err_kind::MJ_ERR_KIND_WRITE_FAILURE,
            _ => return Err(()),
        })
    }
}

/// Returns the error's kind
#[no_mangle]
pub unsafe extern "C" fn mj_err_get_kind() -> mj_err_kind {
    LAST_ERROR
        .with_borrow(|x| {
            x.as_ref()
                .and_then(|x| mj_err_kind::try_from(x.kind()).ok())
        })
        .unwrap_or(mj_err_kind::MJ_ERR_KIND_UNKNOWN)
}
