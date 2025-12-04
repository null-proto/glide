use std::{env, fs, path::Path};

use curl::easy::{self};

fn main() {
  let target = env::var("OUT_DIR").unwrap();

  let mut curl = easy::Easy::new();

  curl.url("https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js").unwrap();

  eprintln!("OUTDIR: {}" , target);

  curl.write_function(move |htmx| {
    let file_ = Path::new(&target).join("tmp_htmx.js");
    fs::write(file_ , htmx).unwrap();
    Ok(htmx.len())
  }).unwrap()



}
