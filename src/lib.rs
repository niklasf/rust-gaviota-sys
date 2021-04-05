// Copyright 2018 Niklas Fiekas <niklas.fiekas@backscattering.de>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

#![doc(html_root_url = "https://docs.rs/gaviota-sys/0.1.17")]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_int, c_uchar, c_uint};
    use super::*;

    #[test]
    fn test_probe() {
        unsafe {
            tbstats_reset();
            tbcache_restart(1024 * 1024, 50);

            let paths = tbpaths_init();
            let path = CString::new("Gaviota-Tablebases/gtb/gtb4").unwrap();
            let paths = tbpaths_add(paths, path.as_ptr());
            drop(path); // can drop after adding

            let initinfo = tb_init(1, TB_compression_scheme::tb_CP4 as c_int, paths);
            if initinfo.is_null() {
                panic!("tb_init failed");
            }
            assert!(tb_is_initialized() != 0);
            println!("initinfo: {:?}", CStr::from_ptr(initinfo));
            println!("availability: {}", tb_availability());

            let ws = [TB_squares::tb_A1 as c_uint, TB_squares::tb_B1 as c_uint, TB_squares::tb_NOSQUARE as c_uint];
            let wp = [TB_pieces::tb_KING as c_uchar, TB_pieces::tb_ROOK as c_uchar, TB_pieces::tb_NOPIECE as c_uchar];
            let bs = [TB_squares::tb_E5 as c_uint, TB_squares::tb_NOSQUARE as c_uint];
            let bp = [TB_pieces::tb_KING as c_uchar, TB_pieces::tb_NOPIECE as c_uchar];
            let mut info: c_uint = 0;
            let mut pliestomate: c_uint = 0;

            let result = tb_probe_hard(
                TB_sides::tb_WHITE_TO_MOVE as c_uint, // stm
                TB_squares::tb_NOSQUARE as c_uint, // epsq
                TB_castling::tb_NOCASTLE.0, // castles
                ws.as_ptr(),
                bs.as_ptr(),
                wp.as_ptr(),
                bp.as_ptr(),
                &mut info as *mut c_uint,
                &mut pliestomate as *mut c_uint);

            println!("result: {:?} info: {:?} dtm: {:?}", result, info, pliestomate);
            assert!(result != 0);
            assert_eq!(info, TB_return_values::tb_WMATE.0);
            assert_eq!(pliestomate, 29);

            tbpaths_done(paths);
            tb_done();
        }
    }
}
