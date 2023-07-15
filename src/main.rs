// rust std imports
use std::io::{self, Read, Write};
use std::{fs::File, path::Path};

// external imports
use clap::{Arg, ArgAction, Command};
use tes3::esp::Plugin;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> io::Result<()> {
    let args = Command::new("tes3conv")
        .about("Convert TES3 plugins (.esp) into JSON files (.json), and vice-versa.")
        .override_usage("tes3conv \"test.esp\" \"test.json\"")
        .arg_required_else_help(true)
        .args(&[
            Arg::new("COMPACT")
                .help("Compact json output (skip indentation).")
                .long("compact")
                .short('c')
                .action(ArgAction::SetTrue),
            Arg::new("OVERWRITE")
                .help("Overwrite output without making backups.")
                .long("overwrite")
                .short('o')
                .action(ArgAction::SetTrue),
            Arg::new("INPUT")
                .help("Sets the input file. Pass - to use stdin.")
                .value_parser(validate_input_arg)
                .required(true),
            Arg::new("OUTPUT")
                .help("Sets the output file. Omit to use stdout.")
                .value_parser(validate_output_arg)
                .required(false)
                .default_value("")
                .hide_default_value(true),
        ])
        .get_matches();

    // optional args
    let compact = args.get_flag("COMPACT");
    let overwrite = args.get_flag("OVERWRITE");

    // required args
    let input = args.get_one::<String>("INPUT").unwrap().as_ref();
    let output = args.get_one::<String>("OUTPUT").unwrap().as_ref();

    // do conversion
    convert(input, output, compact, overwrite)
}

/// Convert the contents of input and write to output.
/// The output format is inferred from the file extension.
fn convert(input: &Path, output: &Path, compact: bool, overwrite: bool) -> io::Result<()> {
    let mut plugin = parse(input)?;

    // create backups unless explicitly told not to
    if !overwrite && output.exists() {
        backup(output)?;
    }

    // write TES3 data if applicable file extension
    let ext = get_extension(output);
    if matches!(&*ext, "esm" | "esp" | "omwaddon" | "tmp") {
        return plugin.save_path(output);
    }

    // otherwise default to outputting as JSON data
    let contents = if compact {
        serde_json::to_string(&plugin.objects)
    } else {
        serde_json::to_string_pretty(&plugin.objects)
    }
    .map_err(io::Error::from)?;

    if output.as_os_str().is_empty() {
        // write to stdout if no file provided
        io::stdout().write_all(contents.as_bytes())
    } else {
        // otherwise write into the given file
        File::create(output)?.write_all(contents.as_bytes())
    }
}

/// Parse the contents of the given path into a TES3 Plugin.
/// Whether to parse as JSON or binary is inferred from first character.
fn parse(path: &Path) -> io::Result<Plugin> {
    let mut raw_data = vec![];

    if path.as_os_str() == "-" {
        io::stdin().read_to_end(&mut raw_data)?;
    } else {
        File::open(path)?.read_to_end(&mut raw_data)?;
    };

    let mut plugin = Plugin::new();

    match raw_data.first() {
        Some(b'[') => {
            // if it starts with a '[' assume it's a JSON file
            plugin.objects = serde_json::from_slice(&raw_data).map_err(io::Error::from)?;
        }
        Some(b'T') => {
            // if it starts with a 'T' assume it's a TES3 file
            plugin.load_bytes(&raw_data)?;
        }
        _ => {
            // anything else is guaranteed to be invalid input
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid input."));
        }
    }

    // sort objects so that diffs are a little more useful
    plugin.sort_objects();

    Ok(plugin)
}

/// Make a backup file in case something goes wrong. "foo.json" -> "foo.001.json"
fn backup(path: &Path) -> io::Result<u64> {
    let ext = get_extension(path);

    for i in 0..1000 {
        let backup_path = path.with_extension(format!("{i:03}.{ext}"));
        if !backup_path.exists() {
            return std::fs::copy(path, backup_path);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Failed to create backup.",
    ))
}

/// Input can either be "-" (to use stdin) or a JSON/TES3 file.
fn validate_input_arg(arg: &str) -> Result<String, String> {
    if arg != "-" {
        let path = arg.as_ref();
        validate_extension(path)?;
        if !path.exists() {
            return Err(format!("\"{}\" (file does not exist).", path.display()));
        }
    }
    Ok(arg.into())
}

/// Output can either be empty (to use stdout) or a JSON/TES3 file.
fn validate_output_arg(arg: &str) -> Result<String, String> {
    if !arg.is_empty() {
        validate_extension(arg.as_ref())?;
    }
    Ok(arg.into())
}

/// Verify that the given path has a JSON or TES3 extension.
fn validate_extension(path: &Path) -> Result<(), String> {
    let ext = get_extension(path);
    if matches!(&*ext, "esm" | "esp" | "json" | "omwaddon" | "tmp") {
        return Ok(());
    }
    Err(format!("\"{}\" (invalid file type).", path.display()))
}

/// Get a path's file extension as an ascii lowercase string.
fn get_extension(path: &Path) -> String {
    path.extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_ascii_lowercase()
}
