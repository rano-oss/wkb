pub mod types_h {
    pub type __uint64_t = u64;
    pub type __dev_t = ::core::ffi::c_ulong;
    pub type __uid_t = ::core::ffi::c_uint;
    pub type __gid_t = ::core::ffi::c_uint;
    pub type __ino_t = ::core::ffi::c_ulong;
    pub type __mode_t = ::core::ffi::c_uint;
    pub type __nlink_t = ::core::ffi::c_ulong;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
    pub type __time_t = ::core::ffi::c_long;
    pub type __blksize_t = ::core::ffi::c_long;
    pub type __blkcnt_t = ::core::ffi::c_long;
    pub type __syscall_slong_t = ::core::ffi::c_long;
}
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__syscall_slong_t, __time_t};
}
pub mod struct_stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: ::core::ffi::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::struct_timespec_h::timespec;
    use super::types_h::{
        __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
        __syscall_slong_t, __uid_t,
    };
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
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        pub fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
        pub fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    }
}
pub mod mman_h {
    pub const MAP_FAILED: *mut ::core::ffi::c_void =
        -1 as ::core::ffi::c_int as *mut ::core::ffi::c_void;

    use super::types_h::__off64_t;
    extern "C" {
        pub fn mmap(
            __addr: *mut ::core::ffi::c_void,
            __len: usize,
            __prot: ::core::ffi::c_int,
            __flags: ::core::ffi::c_int,
            __fd: ::core::ffi::c_int,
            __offset: __off64_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn munmap(__addr: *mut ::core::ffi::c_void, __len: usize) -> ::core::ffi::c_int;
    }
}
pub mod bits_stat_h {
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
pub mod fcntl_linux_h {
    pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod fcntl_h {
    extern "C" {
        pub fn open(
            __file: *const ::core::ffi::c_char,
            __oflag: ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int;
    }
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod unistd_h {
    extern "C" {
        pub fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
pub mod mman_linux_h {
    pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    pub const MAP_SHARED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;

pub use self::bits_stat_h::__S_IFMT;
use self::fcntl_h::open;
pub use self::fcntl_linux_h::O_RDONLY;
pub use self::mman_h::{mmap, munmap, MAP_FAILED};
pub use self::mman_linux_h::{MAP_SHARED, PROT_READ};
use self::stat_h::fstat;
pub use self::stdbool_h::{false_0, true_0};
use self::stdio_h::{fdopen, fileno};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::struct_stat_h::stat;
pub use self::struct_timespec_h::timespec;
pub use self::types_h::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __syscall_slong_t, __time_t, __uid_t, __uint64_t,
};
use self::unistd_h::close;
pub use self::FILE_h::FILE;
#[no_mangle]
pub unsafe extern "C" fn open_file(mut path: *const ::core::ffi::c_char) -> *mut FILE {
    unsafe {
        if path.is_null() {
            return ::core::ptr::null_mut::<FILE>();
        }
        let mut fd: ::core::ffi::c_int = open(path, O_RDONLY);
        if fd < 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<FILE>();
        }
        let mut stat_buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut err: ::core::ffi::c_int = fstat(fd, &raw mut stat_buf);
        if err != 0 as ::core::ffi::c_int
            || !(stat_buf.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
        {
            close(fd);
            return ::core::ptr::null_mut::<FILE>();
        }
        let mut fp: *mut FILE = fdopen(fd, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
        if fp.is_null() {
            close(fd);
        }
        return fp;
    }
}
static mut lower_map: [::core::ffi::c_uchar; 256] = [
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
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    36 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    37 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    41 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    43 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    44 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    48 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    52 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    54 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    56 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    100 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    102 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    103 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    104 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    105 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    106 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    107 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    108 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    109 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    110 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    111 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    112 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    113 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    114 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    115 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    116 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    117 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    118 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    119 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    120 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    121 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    122 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    91 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    100 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    102 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    103 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    104 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    105 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    106 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    107 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    108 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    109 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    110 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    111 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    112 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    113 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    114 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    115 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    116 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    117 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    118 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    119 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    120 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    121 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    122 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    123 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    124 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    125 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    126 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    127 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    128 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    129 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    130 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    131 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    132 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    133 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    134 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    135 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    136 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    137 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    138 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    139 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    140 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    141 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    142 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    143 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    144 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    145 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    146 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    147 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    148 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    149 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    150 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    151 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    152 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    153 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    154 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    155 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    156 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    157 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    158 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    159 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    160 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    161 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    162 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    163 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    164 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    165 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    166 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    167 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    168 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    169 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    170 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    171 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    172 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    173 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    174 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    175 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    176 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    177 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    178 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    179 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    180 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    181 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    182 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    183 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    184 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    185 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    186 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    187 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    188 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    189 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    190 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    191 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    192 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    193 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    194 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    195 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    196 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    197 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    198 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    199 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    200 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    201 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    202 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    203 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    204 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    205 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    206 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    207 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    208 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    209 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    210 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    211 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    212 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    213 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    214 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    215 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    216 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    217 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    218 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    219 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    220 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    221 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    222 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    223 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    224 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    225 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    226 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    227 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    228 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    229 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    230 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    231 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    232 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    233 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    234 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    235 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    236 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    237 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    238 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    239 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    240 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    241 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    242 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    243 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    244 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    245 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    246 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    247 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    248 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    249 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    250 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    251 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    252 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    253 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    254 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn to_lower(mut c: ::core::ffi::c_char) -> ::core::ffi::c_char {
    unsafe {
        return lower_map[c as ::core::ffi::c_uchar as usize] as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn istrcmp(
    mut a: *const ::core::ffi::c_char,
    mut b: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: usize = 0 as usize;
        loop {
            if to_lower(*a.offset(i as isize)) as ::core::ffi::c_int
                != to_lower(*b.offset(i as isize)) as ::core::ffi::c_int
            {
                return to_lower(*a.offset(i as isize)) as ::core::ffi::c_int
                    - to_lower(*b.offset(i as isize)) as ::core::ffi::c_int;
            }
            if *a.offset(i as isize) == 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn istrncmp(
    mut a: *const ::core::ffi::c_char,
    mut b: *const ::core::ffi::c_char,
    mut n: usize,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: usize = 0 as usize;
        while i < n {
            if to_lower(*a.offset(i as isize)) as ::core::ffi::c_int
                != to_lower(*b.offset(i as isize)) as ::core::ffi::c_int
            {
                return to_lower(*a.offset(i as isize)) as ::core::ffi::c_int
                    - to_lower(*b.offset(i as isize)) as ::core::ffi::c_int;
            }
            if *a.offset(i as isize) == 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}

// New Rust file utilities
use memmap2::Mmap;
use std::fs::File;
use std::io;
use std::path::Path;

/// Memory-mapped file wrapper with automatic cleanup
pub struct MappedFile {
    mmap: Mmap,
}

impl MappedFile {
    /// Create a new memory-mapped file
    pub fn new(file: &File) -> io::Result<Self> {
        let mmap = unsafe { Mmap::map(file)? };
        Ok(MappedFile { mmap })
    }

    /// Get the mapped data as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.mmap
    }

    /// Get the mapped data as a C string pointer (for FFI compatibility)
    pub fn as_ptr(&self) -> *const ::core::ffi::c_char {
        self.mmap.as_ptr() as *const ::core::ffi::c_char
    }

    /// Get the size of the mapped file
    pub fn len(&self) -> usize {
        self.mmap.len()
    }

    /// Check if the mapped file is empty
    pub fn is_empty(&self) -> bool {
        self.mmap.is_empty()
    }
}

/// Open a file and verify it's a regular file
pub fn open_regular_file(path: &Path) -> io::Result<File> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "not a regular file",
        ));
    }
    Ok(file)
}

/// Open a file from a C string path
pub unsafe fn open_file_from_cstr(path: *const ::core::ffi::c_char) -> io::Result<File> {
    if path.is_null() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "null path"));
    }
    let path_str = std::ffi::CStr::from_ptr(path)
        .to_str()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid UTF-8 in path"))?;
    open_regular_file(Path::new(path_str))
}
