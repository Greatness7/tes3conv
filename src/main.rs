#![feature(osstring_ascii)]

// rust std imports
use std::ffi::{OsStr, OsString};
use std::io::{self, Read, Write};
use std::{fs::File, path::Path};

// external imports
use clap::{App, AppSettings, Arg};
use es3::esp::Plugin;

fn main() -> io::Result<()> {
    let args = App::new("tes3conv")
        .version("0.0.1")
        .usage("tes3conv \"test.esp\" \"test.json\"")
        .args(&[
            Arg::with_name("MINIMIZE")
                .help("Minimize json output (skip indentation).")
                .long("minimize")
                .short("m")
                .takes_value(false),
            Arg::with_name("OVERWRITE")
                .help("Overwrite output without making backups.")
                .long("overwrite")
                .short("o")
                .takes_value(false),
            Arg::with_name("INPUT")
                .help("Sets the input file. Omit to use stdin.")
                .validator_os(validate_input_arg),
            Arg::with_name("OUTPUT")
                .help("Sets the output file. Omit to use stdout.")
                .validator_os(validate_output_arg),
        ])
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // optional args
    let minimize = args.is_present("MINIMIZE");
    let overwrite = args.is_present("OVERWRITE");

    // required args
    let input = args.value_of_os("INPUT").unwrap_or_default();
    let output = args.value_of_os("OUTPUT").unwrap_or_default();

    // do conversion
    convert(input.as_ref(), output.as_ref(), minimize, overwrite)
}

/// Input can either be empty (to use stdin) or a JSON/TES3 file.
fn validate_input_arg(arg: &OsStr) -> Result<(), OsString> {
    if !arg.is_empty() {
        validate_output_arg(arg)?;
        let path = Path::new(arg);
        if !path.exists() {
            return Err(format!("\"{}\" (file does not exist).", path.display()).into());
        }
    }
    Ok(())
}

/// Output can either be empty (to use stdout) or a JSON/TES3 file.
fn validate_output_arg(arg: &OsStr) -> Result<(), OsString> {
    if !arg.is_empty() {
        validate_extension(arg.as_ref())?;
    }
    Ok(())
}

/// Verify that the given path has a JSON or TES3 extension.
fn validate_extension(path: &Path) -> Result<(), OsString> {
    let ext = path.extension().unwrap_or_default().to_ascii_lowercase();
    if ext != "esp" && ext != "esm" && ext != "json" {
        return Err(format!("\"{}\" (invalid file type).", path.display()).into());
    }
    Ok(())
}

/// Parse the contents of the given path into a TES3 Plugin.
/// Whether to parse as JSON or binary is inferred from first character.
fn parse(path: &Path) -> io::Result<Plugin> {
    let mut raw_data = vec![];

    if path.as_os_str().is_empty() {
        io::stdin().read_to_end(&mut raw_data)?;
    } else {
        File::open(path)?.read_to_end(&mut raw_data)?;
    };

    let mut plugin = Plugin::new();

    match raw_data.get(0) {
        // if it starts with a '{' assume it's a JSON file
        Some(b'{') => plugin = serde_json::from_slice(&raw_data).unwrap(),
        // if it starts with a 'T' assume it's a TES3 file
        Some(b'T') => plugin.load_bytes(raw_data)?,
        // anything else is guaranteed to be invalid input
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid input.")),
    }

    // sort objects so that diffs are a little more useful
    plugin.sort();

    Ok(plugin)
}

/// Make a backup file in case something goes wrong. "foo.json" -> "foo.001.json"
fn backup(path: &Path) -> io::Result<u64> {
    let ext = path.extension().unwrap().to_string_lossy();

    for i in 0..u16::MAX {
        let backup_path = path.with_extension(format!("{:03}.{}", i, ext));
        if !backup_path.exists() {
            return std::fs::copy(path, backup_path);
        }
    }

    Err(io::Error::new(io::ErrorKind::Other, "Failed to create backup."))
}

/// Convert the contents of input and write to output.
/// The output format is inferred from the file extension.
fn convert(input: &Path, output: &Path, minimize: bool, overwrite: bool) -> io::Result<()> {
    let mut plugin = parse(input)?;

    if !overwrite && output.exists() {
        backup(output)?;
    }

    let ext = output.extension().unwrap_or_default().to_ascii_lowercase();
    if ext == "esp" || ext == "esm" {
        return plugin.save_path(output);
    }

    match if minimize {
        serde_json::to_string(&plugin)
    } else {
        serde_json::to_string_pretty(&plugin)
    } {
        // write to stdout if no file provided
        Ok(s) if output.as_os_str().is_empty() => io::stdout().write_all(s.as_bytes()),
        // otherwise write into the given file
        Ok(s) => File::create(output)?.write_all(s.as_bytes()),
        // convert serde errors into io errors
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("{}", e))),
    }
}
