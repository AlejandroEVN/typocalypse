use std::{env, ffi::OsString};

#[derive(Debug, Clone)]
enum CliArg {
    Text(String),
    Limit(usize),
    Path(String),
}

#[derive(Debug, PartialEq)]
enum CliFlag {
    Text,
    Limit,
    Path,
}

impl CliFlag {
    fn from_os_str(s: &OsString) -> Result<CliFlag, String> {
        let flag = s
            .to_str()
            .ok_or_else(|| "error: flag contains invalid unicode")?;

        match flag {
            "-t" => Ok(CliFlag::Text),
            "-l" => Ok(CliFlag::Limit),
            "-p" => Ok(CliFlag::Path),
            _ => Err(format!("error: unexpected argument: {flag}")),
        }
    }

    const fn to_raw(&self) -> &'static str {
        match self {
            CliFlag::Text => "-t",
            CliFlag::Limit => "-l",
            CliFlag::Path => "-p",
        }
    }
}

#[derive(Debug)]
pub struct Options {
    pub text: String,
    pub limit: usize,
    pub path: Option<String>,
}

pub const DEFAULT_TEXT: &str = "Lorem ipsum dolor sit amet";

impl Default for Options {
    fn default() -> Self {
        Self {
            text: DEFAULT_TEXT.to_string(),
            limit: 30,
            path: Default::default(),
        }
    }
}

pub fn parse_args() -> Options {
    let mut args = env::args_os();

    let program = args
        .next()
        .and_then(|p| p.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());

    let mut parsed: Vec<CliArg> = Vec::new();

    let mut iter = args.peekable();

    while let Some(os_str) = iter.next() {
        let flag = match CliFlag::from_os_str(&os_str) {
            Ok(cli_flag) => cli_flag,
            Err(err) => {
                eprintln!("{}", err);
                print_usage(&program);
                std::process::exit(2);
            }
        };

        let value = match iter.peek() {
            Some(value) => value,
            None => {
                eprintln!("error: flag {:?} requires a value", flag);
                print_usage(&program);
                std::process::exit(2);
            }
        };

        match parse_flag(&flag, &value) {
            Ok(cli_arg) => parsed.push(cli_arg),
            Err(err) => {
                eprintln!("{}\n", err);
                print_usage(&program);
                std::process::exit(2);
            }
        }

        iter.next();
    }

    let mut options = Options::default();
    for cli_arg in parsed {
        match cli_arg {
            CliArg::Text(text) => options.text = text,
            CliArg::Limit(limit) => options.limit = limit,
            CliArg::Path(path) => options.path = Some(path),
        }
    }

    options
}

fn parse_flag(flag: &CliFlag, value: &OsString) -> Result<CliArg, String> {
    let value = value
        .to_str()
        .ok_or_else(|| "error: value contains invalid unicode")?;

    match flag {
        CliFlag::Text => Ok(CliArg::Text(value.to_string())),
        CliFlag::Limit => {
            let limit = value
                .parse::<usize>()
                .map_err(|_| format!("error: -l expects a number, got {value}"))?;
            Ok(CliArg::Limit(limit))
        }
        CliFlag::Path => Ok(CliArg::Path(value.to_string())),
    }
}

struct ArgDef {
    flag: &'static str,
    value_name: &'static str,
    description: &'static str,
}

const ARG_DEFS: &[ArgDef] = &[
    ArgDef {
        flag: CliFlag::Text.to_raw(),
        value_name: "TEXT",
        description: "Raw text to practice with",
    },
    ArgDef {
        flag: CliFlag::Limit.to_raw(),
        value_name: "LIMIT",
        description: "Word limit (default: 30)",
    },
    ArgDef {
        flag: CliFlag::Path.to_raw(),
        value_name: "PATH",
        description: "File path to read from",
    },
];

fn print_usage(program: &str) {
    eprintln!("Usage: {program} [OPTIONS]\n");
    eprintln!("Options:");
    for arg in ARG_DEFS {
        eprintln!(
            "  {} <{:<10} {}",
            arg.flag,
            format!("{}>", arg.value_name),
            arg.description
        );
    }
}
