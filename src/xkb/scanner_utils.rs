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
    pub use crate::xkb::shared_types::*;
}
pub mod atom_h {
    pub use crate::xkb::shared_types::*;

    extern "C" {
        pub type atom_table;
    }
}

pub mod xkbcommon_h {
    pub use crate::xkb::shared_types::*;
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
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use crate::xkb::shared_types::darray_size_t;
pub use self::internal::__va_list_tag;
pub use self::scanner_utils_h::{scanner, scanner_loc};
pub use self::xkbcommon_h::{
    xkb_log_level, xkb_rule_names, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
    XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
pub unsafe fn scanner_token_location(mut s: *mut scanner) -> scanner_loc {
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
            ptr = crate::xkb::utils::byte_memchr(ptr, b'\n', last.offset_from(ptr) as i64 as usize);
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
