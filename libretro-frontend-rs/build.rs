// use std::env;
// use std::path::PathBuf;

fn main() {
    const LIBRETRO_HEADER_URL: &str = "https://raw.githubusercontent.com/libretro/libretro-common/master/include/libretro.h";
    const LIBRETRO_HEADER_FILE: &str = "include/libretro.h";
    const LIBRETRO_BINDINGS_FILE: &str = "src/libretro.rs";

    // create the include directory
    std::fs::create_dir_all("include").unwrap();

    // use reqwest to download the file into $LIBRETRO_HEADER_FILE
    reqwest::blocking::get(LIBRETRO_HEADER_URL)
        .unwrap()
        .error_for_status()
        .unwrap()
        .copy_to(&mut std::fs::File::create(LIBRETRO_HEADER_FILE).unwrap())
        .unwrap();

    // use bindgen to generate the bindings into "src/libretro.rs"
    let bindings = bindgen::Builder::default()
        .header(LIBRETRO_HEADER_FILE)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(LIBRETRO_BINDINGS_FILE)
        .expect("Couldn't write bindings!");
}
