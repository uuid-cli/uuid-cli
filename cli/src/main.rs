use clap::Parser;
use core_lib as core;

/// Simple CLI to generate UUID values (v1, v4, v5, 6, v7).
#[derive(Parser, Debug)]
#[command(author, version, about = "Generate UUID (multiple versions)", long_about = None, disable_version_flag = true)]
struct Args {
    /// Number of UUIDs to generate
    #[arg(short = 'n', long = "count", default_value_t = 1)]
    count: usize,

    /// Output without hyphens
    #[arg(long = "no-hyphen")]
    no_hyphen: bool,

    /// Wrap UUIDs in braces
    #[arg(long = "braced")]
    braced: bool,

    /// Uppercase hex letters
    #[arg(long = "upper")]
    upper: bool,

    /// UUID version to generate (supported: 1, 4, 5, 6, 7)
    #[arg(long = "uuid-version", default_value_t = 7)]
    uuid_version: u8,

    /// Name for version 5 UUID (required for v5)
    #[arg(long = "name")]
    name: Option<String>,

    /// Namespace for version 5 UUID: dns, url, oid, x500 (default: dns)
    #[arg(long = "namespace", default_value = "dns")]
    namespace: String,
}

fn main() {
    let args = Args::parse();

    match args.uuid_version {
        1 | 4 | 5 | 6 | 7 => (),
        other => {
            eprintln!("unsupported version: {}", other);
            std::process::exit(2);
        }
    };

    if args.uuid_version == 5 && args.name.is_none() {
        eprintln!("version 5 requires --name <NAME>");
        std::process::exit(2);
    }

    for _ in 0..args.count {
        let u = match args.uuid_version {
            1 => core::generate_v1(),
            4 => core::generate_v4(),
            5 => {
                // Safe to unwrap because we validated above
                let name = args.name.as_ref().unwrap();
                core::generate_v5(name, &args.namespace)
            }
            6 => core::generate_v6(),
            7 => core::generate_v7(),
            other => {
                eprintln!("unsupported version: {}", other);
                std::process::exit(2);
            }
        };

        println!(
            "{}",
            core::format_uuid(u, args.no_hyphen, args.braced, args.upper)
        );
    }
}
