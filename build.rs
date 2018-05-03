extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::builder()
        .header("wrapper.h")
        .clang_arg("-IGaviota-Tablebases")
        .clang_arg("-IGaviota-Tablebases/sysport")
        .whitelist_type("TB_STATS")
        .whitelist_var("tb_MAXPATHLEN")
        .whitelist_function("tb_availability")
        .whitelist_function("tb_done")
        .whitelist_function("tb_indexmemory")
        .whitelist_function("tb_init")
        .whitelist_function("tb_is_initialized")
        .whitelist_function("tb_probe_WDL_hard")
        .whitelist_function("tb_probe_WDL_soft")
        .whitelist_function("tb_probe_hard")
        .whitelist_function("tb_probe_soft")
        .whitelist_function("tb_restart")
        .whitelist_function("tbcache_done")
        .whitelist_function("tbcache_flush")
        .whitelist_function("tbcache_init")
        .whitelist_function("tbcache_is_on")
        .whitelist_function("tbcache_restart")
        .whitelist_function("tbpaths_add")
        .whitelist_function("tbpaths_done")
        .whitelist_function("tbpaths_getmain")
        .whitelist_function("tbpaths_init")
        .whitelist_function("tbstats_get")
        .whitelist_function("tbstats_reset")
        .whitelist_type("TB_castling")
        .whitelist_type("TB_compression_scheme")
        .whitelist_type("TB_mask_values")
        .whitelist_type("TB_pieces")
        .whitelist_type("TB_return_values")
        .whitelist_type("TB_sides")
        .whitelist_type("TB_squares")
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
}
