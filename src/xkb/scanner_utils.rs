pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: u32,
        pub fp_offset: u32,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}

pub mod context_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct xkb_context {
        pub refcnt: i32,
        pub log_fn: Option<
            unsafe extern "C" fn(
                *mut xkb_context,
                xkb_log_level,
                *const i8,
                ::core::ffi::VaList,
            ) -> (),
        >,
        pub log_level: xkb_log_level,
        pub log_verbosity: i32,
        pub user_data: *mut ::core::ffi::c_void,
        pub names_dflt: xkb_rule_names,
        pub includes: C2Rust_Unnamed_0,
        pub failed_includes: C2Rust_Unnamed,
        pub atom_table: *mut atom_table,
        pub x11_atom_cache: *mut ::core::ffi::c_void,
        pub text_buffer: [i8; 2048],
        pub text_next: usize,
        #[bitfield(name = "use_environment_names", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "use_secure_getenv", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "pending_default_includes", ty = "bool", bits = "2..=2")]
        pub use_environment_names_use_secure_getenv_pending_default_includes: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut i8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_0 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut i8,
    }

    use super::atom_h::atom_table;
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
}
pub mod atom_h {
    extern "C" {
        pub type atom_table;
    }
}
pub mod darray_h {
    pub type darray_size_t = u32;
}
pub mod xkbcommon_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rule_names {
        pub rules: *const i8,
        pub model: *const i8,
        pub layout: *const i8,
        pub variant: *const i8,
        pub options: *const i8,
    }
    pub type xkb_log_level = u32;
    pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;
    pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
    pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
    pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
    pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;
}
pub mod scanner_utils_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct scanner_loc {
        pub line: usize,
        pub column: usize,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct scanner {
        pub pos: usize,
        pub len: usize,
        pub s: *const i8,
        pub buf: [i8; 1024],
        pub buf_pos: usize,
        pub token_pos: usize,
        pub cached_pos: usize,
        pub cached_loc: scanner_loc,
        pub file_name: *const i8,
        pub ctx: *mut xkb_context,
        pub priv_0: *mut ::core::ffi::c_void,
    }

    use super::context_h::xkb_context;
}
pub mod string_h {

    extern "C" {
        pub fn memchr(
            __s: *const ::core::ffi::c_void,
            __c: i32,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
    }
}

pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
pub use self::internal::__va_list_tag;
pub use self::scanner_utils_h::{scanner, scanner_loc};
use self::string_h::memchr;
pub use self::xkbcommon_h::{
    xkb_log_level, xkb_rule_names, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
    XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
#[no_mangle]
pub unsafe extern "C" fn scanner_token_location(mut s: *mut scanner) -> scanner_loc {
    unsafe {
        let mut line: usize = 0;
        let mut column: usize = 0;
        let mut line_pos: usize = 0 as usize;
        if (*s).cached_pos > (*s).token_pos {
            (*s).cached_pos = 0 as usize;
            (*s).cached_loc.column = 1 as usize;
            (*s).cached_loc.line = (*s).cached_loc.column;
        }
        line = (*s).cached_loc.line;
        let mut ptr: *const i8 = (*s).s.offset((*s).cached_pos as isize);
        let mut last: *const i8 = (*s).s.offset((*s).token_pos as isize);
        loop {
            ptr = memchr(
                ptr as *const ::core::ffi::c_void,
                '\n' as i32,
                last.offset_from(ptr) as i64 as usize,
            ) as *const i8;
            if ptr.is_null() {
                break;
            }
            line = line.wrapping_add(1);
            ptr = ptr.offset(1);
            line_pos = ptr.offset_from((*s).s) as i64 as usize;
        }
        if line == (*s).cached_loc.line {
            column = (*s)
                .cached_loc
                .column
                .wrapping_add((*s).token_pos.wrapping_sub((*s).cached_pos));
        } else {
            column = (*s)
                .token_pos
                .wrapping_sub(line_pos)
                .wrapping_add(1 as usize);
        }
        let mut loc: scanner_loc = scanner_loc {
            line: line,
            column: column,
        };
        (*s).cached_pos = (*s).token_pos;
        (*s).cached_loc = loc;
        return loc;
    }
}
