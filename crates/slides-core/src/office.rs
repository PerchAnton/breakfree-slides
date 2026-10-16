use crate::Result;
use serde::Serialize;
use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, PartialEq)]
pub enum Launch {
    New,
    Open(PathBuf),
    Capabilities,
    Version,
    UiSmoke(PathBuf),
}
pub fn parse(args: impl IntoIterator<Item = OsString>) -> Result<Launch> {
    let args: Vec<_> = args.into_iter().collect();
    match args.as_slice() {
        [] => Ok(Launch::New),
        [a] if a == "--new" => Ok(Launch::New),
        [a] if a == "--capabilities" => Ok(Launch::Capabilities),
        [a] if a == "--version" => Ok(Launch::Version),
        [a, p] if a == "--open" || a == "--ui-smoke" => {
            let p = PathBuf::from(p);
            if !p.is_absolute() {
                return Err("Expected an absolute local path".into());
            }
            if a == "--open" {
                Ok(Launch::Open(p))
            } else {
                Ok(Launch::UiSmoke(p))
            }
        }
        _=>Err("Usage: breakfree-slides [--new | --open <absolute_path> | --capabilities | --version | --ui-smoke <absolute_directory>]".into())
    }
}
#[derive(Serialize)]
struct Format<'a> {
    id: &'a str,
    extensions: [&'a str; 1],
    read: bool,
    write: bool,
}
#[derive(Serialize)]
struct Capabilities<'a> {
    protocol_version: u32,
    app_id: &'a str,
    app_version: &'a str,
    display_name: &'a str,
    capabilities: [&'a str; 2],
    formats: [Format<'a>; 1],
}
pub fn capabilities(version: &str) -> String {
    serde_json::to_string_pretty(&Capabilities {
        protocol_version: 1,
        app_id: "breakfree.slides",
        app_version: version,
        display_name: "BreakFree Slides",
        capabilities: ["new", "open"],
        formats: [Format {
            id: "breakfree.slides.native",
            extensions: ["bfslide"],
            read: true,
            write: true,
        }],
    })
    .expect("Static capability metadata is serializable")
}
