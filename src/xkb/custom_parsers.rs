// use f128; // f128 is unstable, replaced with f64
pub mod types_h {
    pub type __uint32_t = u32;
    pub type __uint64_t = u64;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
}

pub mod getopt_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct option {
        pub name: *const ::core::ffi::c_char,
        pub has_arg: ::core::ffi::c_int,
        pub flag: *mut ::core::ffi::c_int,
        pub val: ::core::ffi::c_int,
    }
    pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    extern "C" {
        pub fn getopt_long(
            ___argc: ::core::ffi::c_int,
            ___argv: *const *mut ::core::ffi::c_char,
            __shortopts: *const ::core::ffi::c_char,
            __longopts: *const option,
            __longind: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
pub mod stdint_uintn_h {
    pub type u32 = __uint32_t;
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint32_t, __uint64_t};
}
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: __uint64_t,
        pub _unused2: [::core::ffi::c_char; 8],
    }
    pub type _IO_lock_t = ();
    use super::types_h::{__off64_t, __off_t, __uint64_t};
    extern "C" {
        pub type _IO_wide_data;
        pub type _IO_codecvt;
        pub type _IO_marker;
    }
}
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
pub mod bench_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct bench_time {
        pub seconds: ::core::ffi::c_long,
        pub nanoseconds: ::core::ffi::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct bench {
        pub start: bench_time,
        pub stop: bench_time,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct estimate {
        pub elapsed: ::core::ffi::c_longlong,
        pub stdev: ::core::ffi::c_longlong,
    }
    extern "C" {
        pub fn bench_start2(bench: *mut bench);
        pub fn bench_stop2(bench: *mut bench);
        pub fn bench_elapsed(bench: *const bench, result: *mut bench_time);
        pub fn predictPerturbed(t1: *const bench_time, t2: *const bench_time, est: *mut estimate);
    }
}
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        pub static mut stderr: *mut FILE;
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn is_xdigit(mut ch: ::core::ffi::c_char) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int >= '0' as i32
                && ch as ::core::ffi::c_int <= '9' as i32
                || ch as ::core::ffi::c_int >= 'a' as i32
                    && ch as ::core::ffi::c_int <= 'f' as i32
                || ch as ::core::ffi::c_int >= 'A' as i32
                    && ch as ::core::ffi::c_int <= 'F' as i32;
        }
    }

    use super::FILE_h::FILE;
    // map_file/unmap_file removed - use crate::xkb::utils::MappedFile instead
}
pub mod utils_numbers_h {

    use super::stdint_uintn_h::uint64_t;
    use super::utils_h::is_xdigit;
    #[inline]
    pub unsafe extern "C" fn parse_dec_to_uint64_t(
        mut s: *const ::core::ffi::c_char,
        mut len: usize,
        mut out: *mut uint64_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut result: uint64_t = 0 as uint64_t;
            let mut i: usize = 0;
            i = 0 as usize;
            while i < len
                && ((*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                    as ::core::ffi::c_uchar as ::core::ffi::c_uint)
                    < 10 as ::core::ffi::c_uint
                && result <= (18446744073709551615 as uint64_t).wrapping_div(10 as uint64_t)
                && result.wrapping_mul(10 as uint64_t)
                    <= (18446744073709551615 as uint64_t).wrapping_sub(
                        (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                            as ::core::ffi::c_uchar as uint64_t,
                    )
            {
                result = result.wrapping_mul(10 as uint64_t).wrapping_add(
                    (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as uint64_t,
                );
                i = i.wrapping_add(1);
            }
            *out = result as uint64_t;
            return if i >= len
                || (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                    as ::core::ffi::c_uchar as ::core::ffi::c_uint
                    >= 10 as ::core::ffi::c_uint
            {
                i as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
    }
    pub static mut digits__: [::core::ffi::c_uchar; 256] = [
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    #[inline]
    pub unsafe extern "C" fn parse_hex_to_uint32_t(
        mut s: *const ::core::ffi::c_char,
        mut len: usize,
        mut out: *mut u32,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut result: u32 = 0 as u32;
            let mut i: usize = 0 as usize;
            while i < len
                && (digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_uint)
                    < 16 as ::core::ffi::c_uint
                && result <= 4294967295 as u32 >> 4 as ::core::ffi::c_int
            {
                result = result.wrapping_mul(16 as u32).wrapping_add(
                    digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u32,
                );
                i = i.wrapping_add(1);
            }
            *out = result as u32;
            return if i >= len || !is_xdigit(*s.offset(i as isize)) {
                i as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
    }
    #[inline]
    pub unsafe extern "C" fn parse_hex_to_uint64_t(
        mut s: *const ::core::ffi::c_char,
        mut len: usize,
        mut out: *mut uint64_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut result: uint64_t = 0 as uint64_t;
            let mut i: usize = 0 as usize;
            while i < len
                && (digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_uint)
                    < 16 as ::core::ffi::c_uint
                && result <= 18446744073709551615 as uint64_t >> 4 as ::core::ffi::c_int
            {
                result = result.wrapping_mul(16 as uint64_t).wrapping_add(
                    digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as uint64_t,
                );
                i = i.wrapping_add(1);
            }
            *out = result as uint64_t;
            return if i >= len || !is_xdigit(*s.offset(i as isize)) {
                i as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
    }
}
pub mod getopt_core_h {
    extern "C" {
        pub static mut optarg: *mut ::core::ffi::c_char;
    }
}
pub mod assert_h {
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub mod stdbool_h {
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod stdlib_h {
    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    extern "C" {
        pub fn atof(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
        pub fn strtol(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
        pub fn exit(__status: ::core::ffi::c_int) -> !;
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod config_h {
    pub const EXIT_INVALID_USAGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;

use self::assert_h::__assert_fail;
pub use self::bench_h::{
    bench, bench_elapsed, bench_start2, bench_stop2, bench_time, estimate, predictPerturbed,
};
pub use self::config_h::EXIT_INVALID_USAGE;
use self::getopt_core_h::optarg;
pub use self::getopt_ext_h::{getopt_long, no_argument, option, required_argument};
pub use self::stdbool_h::false_0;
pub use self::stdint_uintn_h::{u32, uint64_t};
use self::stdio_h::{fclose, fopen, fprintf, printf, stderr};
pub use self::stdlib_h::{atof, exit, strtol, EXIT_SUCCESS};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__off64_t, __off_t, __uint32_t, __uint64_t};
pub use self::utils_h::is_xdigit;
pub use self::utils_numbers_h::{
    digits__, parse_dec_to_uint64_t, parse_hex_to_uint32_t, parse_hex_to_uint64_t,
};
pub use self::FILE_h::FILE;
pub const OPT_STDEV: options = 0;
pub type options = ::core::ffi::c_uint;
#[no_mangle]
pub static mut DEFAULT_STDEV: ::core::ffi::c_double = 0.05f64;
unsafe extern "C" fn usage(mut argv: *mut *mut ::core::ffi::c_char) {
    unsafe {
        printf(
            b"Usage: %s [OPTIONS]\n\nBenchmark compilation of the given RMLVO\n\nOptions:\n --help\n    Print this help and exit\n --stdev\n    Minimal relative standard deviation (percentage) to reach.\n    (default: %f)\n\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            DEFAULT_STDEV * 100 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
    }
}
unsafe extern "C" fn print_stats(
    mut stdev: ::core::ffi::c_double,
    mut max_iterations: ::core::ffi::c_uint,
    mut elapsed: *mut bench_time,
    mut bench: *mut bench,
    mut est: *mut estimate,
) {
    unsafe {
        let mut total_elapsed: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        bench_elapsed(bench, &raw mut total_elapsed);
        fprintf(
            stderr,
            b"mean: %lld \xC2\xB5s; stdev: %Lf%% (target: %f%%); last run: parsed %u times in %ld.%06lds; total time: %ld.%06lds\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*est).elapsed / 1000 as ::core::ffi::c_longlong,
            ((*est).stdev as f64) * (100.0 as f64)
                / ((*est).elapsed as f64),
            stdev * 100 as ::core::ffi::c_int as ::core::ffi::c_double,
            max_iterations,
            (*elapsed).seconds,
            (*elapsed).nanoseconds / 1000 as ::core::ffi::c_long,
            total_elapsed.seconds,
            total_elapsed.nanoseconds / 1000 as ::core::ffi::c_long,
        );
    }
}
unsafe extern "C" fn parse_keysym_hex(
    mut s: *const ::core::ffi::c_char,
    mut out: *mut u32,
) -> bool {
    unsafe {
        let mut result: u32 = 0 as u32;
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < 8 as ::core::ffi::c_uint
            && *s.offset(i as isize) as ::core::ffi::c_int != '\0' as i32
        {
            result <<= 4 as ::core::ffi::c_int;
            if '0' as i32 <= *s.offset(i as isize) as ::core::ffi::c_int
                && *s.offset(i as isize) as ::core::ffi::c_int <= '9' as i32
            {
                result = result.wrapping_add(
                    (*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as u32,
                );
            } else if 'a' as i32 <= *s.offset(i as isize) as ::core::ffi::c_int
                && *s.offset(i as isize) as ::core::ffi::c_int <= 'f' as i32
            {
                result = result.wrapping_add(
                    (10 as ::core::ffi::c_int + *s.offset(i as isize) as ::core::ffi::c_int
                        - 'a' as i32) as u32,
                );
            } else if 'A' as i32 <= *s.offset(i as isize) as ::core::ffi::c_int
                && *s.offset(i as isize) as ::core::ffi::c_int <= 'F' as i32
            {
                result = result.wrapping_add(
                    (10 as ::core::ffi::c_int + *s.offset(i as isize) as ::core::ffi::c_int
                        - 'A' as i32) as u32,
                );
            } else {
                return false_0 != 0;
            }
            i = i.wrapping_add(1);
        }
        *out = result;
        return *s.offset(i as isize) as ::core::ffi::c_int == '\0' as i32
            && i > 0 as ::core::ffi::c_uint;
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut bench: bench = bench {
            start: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
            stop: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
        };
        let mut elapsed: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        let mut est: estimate = estimate {
            elapsed: 0,
            stdev: 0,
        };
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut stdev: ::core::ffi::c_double = DEFAULT_STDEV;
        static mut opts: [option; 3] = [
            option {
                name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
                has_arg: no_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: 'h' as i32,
            },
            option {
                name: b"stdev\0".as_ptr() as *const ::core::ffi::c_char,
                has_arg: required_argument,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: OPT_STDEV as ::core::ffi::c_int,
            },
            option {
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                has_arg: 0 as ::core::ffi::c_int,
                flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
                val: 0 as ::core::ffi::c_int,
            },
        ];
        loop {
            let mut c: ::core::ffi::c_int = 0;
            let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            c = getopt_long(
                argc,
                argv,
                b"h\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut opts as *mut option,
                &raw mut option_index,
            );
            if c == -1 as ::core::ffi::c_int {
                break;
            }
            match c {
                104 => {
                    usage(argv);
                    exit(EXIT_SUCCESS);
                }
                0 => {
                    stdev = atof(optarg) / 100 as ::core::ffi::c_int as ::core::ffi::c_double;
                    if stdev <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        stdev = DEFAULT_STDEV;
                    }
                }
                _ => {
                    usage(argv);
                    exit(EXIT_INVALID_USAGE);
                }
            }
        }
        let mut file: *mut FILE = fopen(
            b"../bench/custom-parsers.c\0".as_ptr() as *const ::core::ffi::c_char,
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if !file.is_null() {
        } else {
            __assert_fail(
                b"file\0".as_ptr() as *const ::core::ffi::c_char,
                b"../bench/custom-parsers.c\0".as_ptr() as *const ::core::ffi::c_char,
                117 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };

        // Convert FILE* to Rust File and map it
        use crate::xkb::utils::MappedFile;
        use std::fs::File;
        use std::os::unix::io::FromRawFd;

        let fd = libc::fileno(file as *mut libc::FILE);
        if fd < 0 {
            fprintf(
                stderr,
                b"Invalid file descriptor\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            exit(1);
        }

        let rust_file = File::from_raw_fd(fd);
        let mapped = match MappedFile::new(&rust_file) {
            Ok(m) => m,
            Err(e) => {
                fprintf(
                    stderr,
                    b"Failed to map file: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    std::ffi::CString::new(e.to_string()).unwrap().as_ptr(),
                );
                std::mem::forget(rust_file);
                exit(1);
            }
        };

        let content = mapped.as_ptr();
        let size = mapped.len();
        if !content.is_null() {
        } else {
            __assert_fail(
                b"content\0".as_ptr() as *const ::core::ffi::c_char,
                b"../bench/custom-parsers.c\0".as_ptr() as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_uint,
                b"int main(int, char **)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        let mut dummy32: u32 = 0 as u32;
        let mut dummy64: uint64_t = 0 as uint64_t;
        let mut max_iterations: ::core::ffi::c_uint = 0;
        printf(b"*** parse_hex_to_uint32_t ***\n\0".as_ptr() as *const ::core::ffi::c_char);
        bench_start2(&raw mut bench);
        let mut _bench: bench = bench {
            start: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
            stop: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
        };
        let mut _t1: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        let mut _t2: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        max_iterations = 1 as ::core::ffi::c_uint;
        bench_start2(&raw mut _bench);
        let mut n: usize = 0 as usize;
        while n < size {
            let mut val: u32 = 0 as u32;
            parse_hex_to_uint32_t(content.offset(n as isize), 8 as usize, &raw mut val);
            ::core::ptr::write_volatile(
                &mut dummy32 as *mut u32,
                ::core::ptr::read_volatile::<u32>(&dummy32 as *const u32).wrapping_add(val),
            );
            n = n.wrapping_add(1);
        }
        bench_stop2(&raw mut _bench);
        bench_elapsed(&raw mut _bench, &raw mut _t1);
        loop {
            bench_start2(&raw mut _bench);
            let mut k: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while k < (2 as ::core::ffi::c_uint).wrapping_mul(max_iterations) {
                let mut n_0: usize = 0 as usize;
                while n_0 < size {
                    let mut val_0: u32 = 0 as u32;
                    parse_hex_to_uint32_t(content.offset(n_0 as isize), 8 as usize, &raw mut val_0);
                    ::core::ptr::write_volatile(
                        &mut dummy32 as *mut u32,
                        ::core::ptr::read_volatile::<u32>(&dummy32 as *const u32)
                            .wrapping_add(val_0),
                    );
                    n_0 = n_0.wrapping_add(1);
                }
                k = k.wrapping_add(1);
            }
            bench_stop2(&raw mut _bench);
            bench_elapsed(&raw mut _bench, &raw mut _t2);
            predictPerturbed(&raw mut _t1, &raw mut _t2, &raw mut est);
            if est.stdev
                < (if 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    > stdev * est.elapsed as ::core::ffi::c_double
                {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    stdev * est.elapsed as ::core::ffi::c_double
                }) as ::core::ffi::c_longlong
            {
                est.elapsed /= max_iterations as ::core::ffi::c_longlong;
                est.stdev /= max_iterations as ::core::ffi::c_longlong;
                elapsed = _t2;
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                break;
            } else {
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                _t1 = _t2;
            }
        }
        bench_stop2(&raw mut bench);
        print_stats(
            stdev,
            max_iterations,
            &raw mut elapsed,
            &raw mut bench,
            &raw mut est,
        );
        printf(b"*** parse_keysym_hex ***\n\0".as_ptr() as *const ::core::ffi::c_char);
        bench_start2(&raw mut bench);
        let mut _bench_0: bench = bench {
            start: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
            stop: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
        };
        let mut _t1_0: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        let mut _t2_0: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        max_iterations = 1 as ::core::ffi::c_uint;
        bench_start2(&raw mut _bench_0);
        let mut n_1: usize = 0 as usize;
        while n_1 < size {
            let mut val_1: u32 = 0 as u32;
            parse_keysym_hex(content.offset(n_1 as isize), &raw mut val_1);
            ::core::ptr::write_volatile(
                &mut dummy32 as *mut u32,
                ::core::ptr::read_volatile::<u32>(&dummy32 as *const u32).wrapping_add(val_1),
            );
            n_1 = n_1.wrapping_add(1);
        }
        bench_stop2(&raw mut _bench_0);
        bench_elapsed(&raw mut _bench_0, &raw mut _t1_0);
        loop {
            bench_start2(&raw mut _bench_0);
            let mut k_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while k_0 < (2 as ::core::ffi::c_uint).wrapping_mul(max_iterations) {
                let mut n_2: usize = 0 as usize;
                while n_2 < size {
                    let mut val_2: u32 = 0 as u32;
                    parse_keysym_hex(content.offset(n_2 as isize), &raw mut val_2);
                    ::core::ptr::write_volatile(
                        &mut dummy32 as *mut u32,
                        ::core::ptr::read_volatile::<u32>(&dummy32 as *const u32)
                            .wrapping_add(val_2),
                    );
                    n_2 = n_2.wrapping_add(1);
                }
                k_0 = k_0.wrapping_add(1);
            }
            bench_stop2(&raw mut _bench_0);
            bench_elapsed(&raw mut _bench_0, &raw mut _t2_0);
            predictPerturbed(&raw mut _t1_0, &raw mut _t2_0, &raw mut est);
            if est.stdev
                < (if 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    > stdev * est.elapsed as ::core::ffi::c_double
                {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    stdev * est.elapsed as ::core::ffi::c_double
                }) as ::core::ffi::c_longlong
            {
                est.elapsed /= max_iterations as ::core::ffi::c_longlong;
                est.stdev /= max_iterations as ::core::ffi::c_longlong;
                elapsed = _t2_0;
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                break;
            } else {
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                _t1_0 = _t2_0;
            }
        }
        bench_stop2(&raw mut bench);
        print_stats(
            stdev,
            max_iterations,
            &raw mut elapsed,
            &raw mut bench,
            &raw mut est,
        );
        printf(b"*** parse_dec_to_uint64_t ***\n\0".as_ptr() as *const ::core::ffi::c_char);
        bench_start2(&raw mut bench);
        let mut _bench_1: bench = bench {
            start: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
            stop: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
        };
        let mut _t1_1: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        let mut _t2_1: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        max_iterations = 1 as ::core::ffi::c_uint;
        bench_start2(&raw mut _bench_1);
        let mut n_3: usize = 0 as usize;
        while n_3 < size {
            let mut val_3: uint64_t = 0 as uint64_t;
            parse_dec_to_uint64_t(
                content.offset(n_3 as isize),
                size.wrapping_sub(n_3),
                &raw mut val_3,
            );
            ::core::ptr::write_volatile(
                &mut dummy64 as *mut uint64_t,
                ::core::ptr::read_volatile::<uint64_t>(&dummy64 as *const uint64_t)
                    .wrapping_add(val_3),
            );
            n_3 = n_3.wrapping_add(1);
        }
        bench_stop2(&raw mut _bench_1);
        bench_elapsed(&raw mut _bench_1, &raw mut _t1_1);
        loop {
            bench_start2(&raw mut _bench_1);
            let mut k_1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while k_1 < (2 as ::core::ffi::c_uint).wrapping_mul(max_iterations) {
                let mut n_4: usize = 0 as usize;
                while n_4 < size {
                    let mut val_4: uint64_t = 0 as uint64_t;
                    parse_dec_to_uint64_t(
                        content.offset(n_4 as isize),
                        size.wrapping_sub(n_4),
                        &raw mut val_4,
                    );
                    ::core::ptr::write_volatile(
                        &mut dummy64 as *mut uint64_t,
                        ::core::ptr::read_volatile::<uint64_t>(&dummy64 as *const uint64_t)
                            .wrapping_add(val_4),
                    );
                    n_4 = n_4.wrapping_add(1);
                }
                k_1 = k_1.wrapping_add(1);
            }
            bench_stop2(&raw mut _bench_1);
            bench_elapsed(&raw mut _bench_1, &raw mut _t2_1);
            predictPerturbed(&raw mut _t1_1, &raw mut _t2_1, &raw mut est);
            if est.stdev
                < (if 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    > stdev * est.elapsed as ::core::ffi::c_double
                {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    stdev * est.elapsed as ::core::ffi::c_double
                }) as ::core::ffi::c_longlong
            {
                est.elapsed /= max_iterations as ::core::ffi::c_longlong;
                est.stdev /= max_iterations as ::core::ffi::c_longlong;
                elapsed = _t2_1;
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                break;
            } else {
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                _t1_1 = _t2_1;
            }
        }
        bench_stop2(&raw mut bench);
        print_stats(
            stdev,
            max_iterations,
            &raw mut elapsed,
            &raw mut bench,
            &raw mut est,
        );
        printf(b"*** strtol, base 10 ***\n\0".as_ptr() as *const ::core::ffi::c_char);
        bench_start2(&raw mut bench);
        let mut _bench_2: bench = bench {
            start: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
            stop: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
        };
        let mut _t1_2: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        let mut _t2_2: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        max_iterations = 1 as ::core::ffi::c_uint;
        bench_start2(&raw mut _bench_2);
        let mut n_5: usize = 0 as usize;
        while n_5 < size {
            ::core::ptr::write_volatile(
                &mut dummy64 as *mut uint64_t,
                ::core::ptr::read_volatile::<uint64_t>(&dummy64 as *const uint64_t).wrapping_add(
                    strtol(
                        content.offset(n_5 as isize),
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        10 as ::core::ffi::c_int,
                    ) as uint64_t,
                ),
            );
            n_5 = n_5.wrapping_add(1);
        }
        bench_stop2(&raw mut _bench_2);
        bench_elapsed(&raw mut _bench_2, &raw mut _t1_2);
        loop {
            bench_start2(&raw mut _bench_2);
            let mut k_2: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while k_2 < (2 as ::core::ffi::c_uint).wrapping_mul(max_iterations) {
                let mut n_6: usize = 0 as usize;
                while n_6 < size {
                    ::core::ptr::write_volatile(
                        &mut dummy64 as *mut uint64_t,
                        ::core::ptr::read_volatile::<uint64_t>(&dummy64 as *const uint64_t)
                            .wrapping_add(strtol(
                                content.offset(n_6 as isize),
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                10 as ::core::ffi::c_int,
                            ) as uint64_t),
                    );
                    n_6 = n_6.wrapping_add(1);
                }
                k_2 = k_2.wrapping_add(1);
            }
            bench_stop2(&raw mut _bench_2);
            bench_elapsed(&raw mut _bench_2, &raw mut _t2_2);
            predictPerturbed(&raw mut _t1_2, &raw mut _t2_2, &raw mut est);
            if est.stdev
                < (if 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    > stdev * est.elapsed as ::core::ffi::c_double
                {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    stdev * est.elapsed as ::core::ffi::c_double
                }) as ::core::ffi::c_longlong
            {
                est.elapsed /= max_iterations as ::core::ffi::c_longlong;
                est.stdev /= max_iterations as ::core::ffi::c_longlong;
                elapsed = _t2_2;
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                break;
            } else {
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                _t1_2 = _t2_2;
            }
        }
        bench_stop2(&raw mut bench);
        print_stats(
            stdev,
            max_iterations,
            &raw mut elapsed,
            &raw mut bench,
            &raw mut est,
        );
        printf(b"*** parse_hex_to_uint64_t ***\n\0".as_ptr() as *const ::core::ffi::c_char);
        bench_start2(&raw mut bench);
        let mut _bench_3: bench = bench {
            start: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
            stop: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
        };
        let mut _t1_3: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        let mut _t2_3: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        max_iterations = 1 as ::core::ffi::c_uint;
        bench_start2(&raw mut _bench_3);
        let mut n_7: usize = 0 as usize;
        while n_7 < size {
            let mut val_5: uint64_t = 0 as uint64_t;
            parse_hex_to_uint64_t(
                content.offset(n_7 as isize),
                size.wrapping_sub(n_7),
                &raw mut val_5,
            );
            ::core::ptr::write_volatile(
                &mut dummy64 as *mut uint64_t,
                ::core::ptr::read_volatile::<uint64_t>(&dummy64 as *const uint64_t)
                    .wrapping_add(val_5),
            );
            n_7 = n_7.wrapping_add(1);
        }
        bench_stop2(&raw mut _bench_3);
        bench_elapsed(&raw mut _bench_3, &raw mut _t1_3);
        loop {
            bench_start2(&raw mut _bench_3);
            let mut k_3: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while k_3 < (2 as ::core::ffi::c_uint).wrapping_mul(max_iterations) {
                let mut n_8: usize = 0 as usize;
                while n_8 < size {
                    let mut val_6: uint64_t = 0 as uint64_t;
                    parse_hex_to_uint64_t(
                        content.offset(n_8 as isize),
                        size.wrapping_sub(n_8),
                        &raw mut val_6,
                    );
                    ::core::ptr::write_volatile(
                        &mut dummy64 as *mut uint64_t,
                        ::core::ptr::read_volatile::<uint64_t>(&dummy64 as *const uint64_t)
                            .wrapping_add(val_6),
                    );
                    n_8 = n_8.wrapping_add(1);
                }
                k_3 = k_3.wrapping_add(1);
            }
            bench_stop2(&raw mut _bench_3);
            bench_elapsed(&raw mut _bench_3, &raw mut _t2_3);
            predictPerturbed(&raw mut _t1_3, &raw mut _t2_3, &raw mut est);
            if est.stdev
                < (if 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    > stdev * est.elapsed as ::core::ffi::c_double
                {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    stdev * est.elapsed as ::core::ffi::c_double
                }) as ::core::ffi::c_longlong
            {
                est.elapsed /= max_iterations as ::core::ffi::c_longlong;
                est.stdev /= max_iterations as ::core::ffi::c_longlong;
                elapsed = _t2_3;
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                break;
            } else {
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                _t1_3 = _t2_3;
            }
        }
        bench_stop2(&raw mut bench);
        print_stats(
            stdev,
            max_iterations,
            &raw mut elapsed,
            &raw mut bench,
            &raw mut est,
        );
        printf(b"*** strtol, base 16 ***\n\0".as_ptr() as *const ::core::ffi::c_char);
        bench_start2(&raw mut bench);
        let mut _bench_4: bench = bench {
            start: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
            stop: bench_time {
                seconds: 0,
                nanoseconds: 0,
            },
        };
        let mut _t1_4: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        let mut _t2_4: bench_time = bench_time {
            seconds: 0,
            nanoseconds: 0,
        };
        max_iterations = 1 as ::core::ffi::c_uint;
        bench_start2(&raw mut _bench_4);
        let mut n_9: usize = 0 as usize;
        while n_9 < size {
            ::core::ptr::write_volatile(
                &mut dummy64 as *mut uint64_t,
                ::core::ptr::read_volatile::<uint64_t>(&dummy64 as *const uint64_t).wrapping_add(
                    strtol(
                        content.offset(n_9 as isize),
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        16 as ::core::ffi::c_int,
                    ) as uint64_t,
                ),
            );
            n_9 = n_9.wrapping_add(1);
        }
        bench_stop2(&raw mut _bench_4);
        bench_elapsed(&raw mut _bench_4, &raw mut _t1_4);
        loop {
            bench_start2(&raw mut _bench_4);
            let mut k_4: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while k_4 < (2 as ::core::ffi::c_uint).wrapping_mul(max_iterations) {
                let mut n_10: usize = 0 as usize;
                while n_10 < size {
                    ::core::ptr::write_volatile(
                        &mut dummy64 as *mut uint64_t,
                        ::core::ptr::read_volatile::<uint64_t>(&dummy64 as *const uint64_t)
                            .wrapping_add(strtol(
                                content.offset(n_10 as isize),
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                16 as ::core::ffi::c_int,
                            ) as uint64_t),
                    );
                    n_10 = n_10.wrapping_add(1);
                }
                k_4 = k_4.wrapping_add(1);
            }
            bench_stop2(&raw mut _bench_4);
            bench_elapsed(&raw mut _bench_4, &raw mut _t2_4);
            predictPerturbed(&raw mut _t1_4, &raw mut _t2_4, &raw mut est);
            if est.stdev
                < (if 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    > stdev * est.elapsed as ::core::ffi::c_double
                {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    stdev * est.elapsed as ::core::ffi::c_double
                }) as ::core::ffi::c_longlong
            {
                est.elapsed /= max_iterations as ::core::ffi::c_longlong;
                est.stdev /= max_iterations as ::core::ffi::c_longlong;
                elapsed = _t2_4;
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                break;
            } else {
                max_iterations = max_iterations.wrapping_mul(2 as ::core::ffi::c_uint);
                _t1_4 = _t2_4;
            }
        }
        bench_stop2(&raw mut bench);
        print_stats(
            stdev,
            max_iterations,
            &raw mut elapsed,
            &raw mut bench,
            &raw mut est,
        );
        // mapped will auto-unmap on drop
        std::mem::forget(rust_file);
        fclose(file);
        return ret;
    }
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
