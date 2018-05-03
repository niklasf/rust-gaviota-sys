#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe() {
        unsafe {
            tbstats_reset();
            tbcache_restart(1024 * 1024, 50);

            let paths = tbpaths_init();
            let paths = tbpaths_add(paths, CString::new("/home/niklas/Projekte/syzygy-tables.info/gaviota").unwrap().as_ptr());

            let initinfo = tb_init(1, TB_compression_scheme_tb_CP4 as i32, paths);
            if initinfo.is_null() {
                panic!("tb_init failed");
            }
            assert!(tb_is_initialized() != 0);
            println!("{:?}", CStr::from_ptr(initinfo));
            println!("availability: {}", tb_availability());

            tbpaths_done(paths);
            tb_done();

            panic!("intended fail");
        }
    }
}
