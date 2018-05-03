#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        unsafe {
            let paths = tbpaths_init();
            let paths = tbpaths_add(paths, CString::new("/home/niklas/Projekte/syzygy-tables.info/gaviota").unwrap().as_ptr());

            let initinfo = tb_init(1, TB_compression_scheme_tb_CP4 as i32, paths);
        }
    }
}
