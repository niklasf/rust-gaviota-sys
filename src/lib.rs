#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_uchar, c_uint};
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
            println!("initinfo: {:?}", CStr::from_ptr(initinfo));
            println!("availability: {}", tb_availability());

            let ws = [TB_squares_tb_A1 as c_uint, TB_squares_tb_B1 as c_uint, TB_squares_tb_NOSQUARE as c_uint];
            let wp = [TB_pieces_tb_KING as c_uchar, TB_pieces_tb_ROOK as c_uchar, TB_pieces_tb_NOPIECE as c_uchar];
            let bs = [TB_squares_tb_E5 as c_uint, TB_squares_tb_NOSQUARE as c_uint];
            let bp = [TB_pieces_tb_KING as c_uchar, TB_pieces_tb_NOPIECE as c_uchar];
            let mut info: c_uint = 0;
            let mut pliestomate: c_uint = 0;

            let result = tb_probe_hard(
                TB_sides_tb_WHITE_TO_MOVE, // stm
                TB_squares_tb_NOSQUARE, // epsq
                TB_castling_tb_NOCASTLE, // castles
                ws.as_ptr(),
                bs.as_ptr(),
                wp.as_ptr(),
                bp.as_ptr(),
                &mut info as *mut c_uint,
                &mut pliestomate as *mut c_uint);

            println!("result: {:?} info: {:?} dtm: {:?}", result, info, pliestomate);
            assert!(result != 0);
            assert_eq!(info, TB_return_values_tb_WMATE);
            assert_eq!(pliestomate, 29);

            tbpaths_done(paths);
            tb_done();
        }
    }
}
