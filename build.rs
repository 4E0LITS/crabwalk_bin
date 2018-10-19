/*
Set locations for config file and search / wordgen plugin dirs
*/

use std::env::{var, current_dir};
use std::fs::{create_dir_all, File};
use std::io::{Write, ErrorKind::NotFound};
use std::path::Path;

fn main() {
    // you can change where plugins / config live and their names by editing the values below:

    let PATH: &str = &current_dir().unwrap().to_str().unwrap().to_owned(); // where these live
    let PLUG: &str = "crabwalk"; // plugin dir name
    let SRCH: &str = "searches"; // search dir name
    let WGEN: &str = "wordgen";  // wordgen dir name
    let CONF: &str = "conf.txt"; // config file name

    let out_dir = var("OUT_DIR").unwrap();

    // generate build files - these will be used by the precompiler to set the locations of
    // the stuff we're creating. if you want to change anything about the plug dirs or
    // config file, you're in the wrong place.
    File::create(&Path::new(&out_dir).join("path")).unwrap().write_all(PATH.as_bytes()).unwrap();
    File::create(&Path::new(&out_dir).join("plug")).unwrap().write_all(PLUG.as_bytes()).unwrap();
    File::create(&Path::new(&out_dir).join("srch")).unwrap().write_all(SRCH.as_bytes()).unwrap();
    File::create(&Path::new(&out_dir).join("wgen")).unwrap().write_all(WGEN.as_bytes()).unwrap();
    File::create(&Path::new(&out_dir).join("conf")).unwrap().write_all(CONF.as_bytes()).unwrap();

    // generate plugin dirs if they don't exist
    create_dir_all(&Path::new(PATH).join(PLUG).join(SRCH)).unwrap();
    create_dir_all(&Path::new(PATH).join(PLUG).join(WGEN)).unwrap();

    // generate configuration file if it doesn't exist
    let confp = Path::new(PATH).join(CONF);
    if let Err(e) = File::open(&confp) {
        if e.kind() == NotFound {
            File::create(&confp).unwrap().write_all(
                b""
            );
        }
    }
}