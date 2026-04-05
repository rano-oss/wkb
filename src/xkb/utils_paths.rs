pub mod utils_paths_h {
    pub const PATH_SEPARATOR: ::core::ffi::c_int = '/' as i32;
}
pub use self::utils_paths_h::PATH_SEPARATOR;
#[no_mangle]
pub unsafe extern "C" fn is_absolute_path(mut path: *const ::core::ffi::c_char) -> bool {
    unsafe {
        return *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == PATH_SEPARATOR;
    }
}
