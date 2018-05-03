extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn generate_bindings() {
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
        .bitfield_enum("TB_return_values")
        .bitfield_enum("TB_castling")
        .rustified_enum("TB_sides")
        .rustified_enum("TB_squares")
        .rustified_enum("TB_pieces")
        .rustified_enum("TB_compression_scheme")
        .prepend_enum_name(false)
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
}

fn compile() {
    cc::Build::new()
        .include("Gaviota-Tablebases")
        .include("Gaviota-Tablebases/sysport")
        .include("Gaviota-Tablebases/compression")
        .include("Gaviota-Tablebases/compression/liblzf")
        .include("Gaviota-Tablebases/compression/zlib")
        .include("Gaviota-Tablebases/compression/lzma")
        .include("Gaviota-Tablebases/compression/huffman")
        .define("Z_PREFIX", None)
        .file("Gaviota-Tablebases/gtb-probe.c")
        .file("Gaviota-Tablebases/gtb-dec.c")
        .file("Gaviota-Tablebases/gtb-att.c")
        .file("Gaviota-Tablebases/sysport/sysport.c")
        .file("Gaviota-Tablebases/compression/wrap.c")
        .file("Gaviota-Tablebases/compression/huffman/hzip.c")
        .file("Gaviota-Tablebases/compression/lzma/LzmaEnc.c")
        .file("Gaviota-Tablebases/compression/lzma/LzmaDec.c")
        .file("Gaviota-Tablebases/compression/lzma/Alloc.c")
        .file("Gaviota-Tablebases/compression/lzma/LzFind.c")
        .file("Gaviota-Tablebases/compression/lzma/Lzma86Enc.c")
        .file("Gaviota-Tablebases/compression/lzma/Lzma86Dec.c")
        .file("Gaviota-Tablebases/compression/lzma/Bra86.c")
        .file("Gaviota-Tablebases/compression/zlib/zcompress.c")
        .file("Gaviota-Tablebases/compression/zlib/uncompr.c")
        .file("Gaviota-Tablebases/compression/zlib/inflate.c")
        .file("Gaviota-Tablebases/compression/zlib/deflate.c")
        .file("Gaviota-Tablebases/compression/zlib/adler32.c")
        .file("Gaviota-Tablebases/compression/zlib/crc32.c")
        .file("Gaviota-Tablebases/compression/zlib/infback.c")
        .file("Gaviota-Tablebases/compression/zlib/inffast.c")
        .file("Gaviota-Tablebases/compression/zlib/inftrees.c")
        .file("Gaviota-Tablebases/compression/zlib/trees.c")
        .file("Gaviota-Tablebases/compression/zlib/zutil.c")
        .file("Gaviota-Tablebases/compression/liblzf/lzf_c.c")
        .file("Gaviota-Tablebases/compression/liblzf/lzf_d.c")
        .compile("libgtb.a");
}

fn main() {
    generate_bindings();
    compile();
}
