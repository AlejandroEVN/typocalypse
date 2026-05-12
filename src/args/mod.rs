use std::env;

#[derive(Debug, Clone, Copy)]
enum CliArgType {
    Text,
    Limit,
    Path,
    Value,
}

#[derive(Debug, Clone)]
struct CliArg {
    raw: String,
    arg_type: CliArgType,
}

struct Options {
    pub text: String,
    pub limit: u8,
    pub path: Option<String>,
}

pub const DEFAULT_TEXT: &str = "Lorem ipsum dolor sit amet";

const HELP: &str = "Something something";

impl Default for Options {
    fn default() -> Self {
        Self {
            text: DEFAULT_TEXT.to_string(),
            limit: 30,
            path: Default::default(),
        }
    }
}

pub fn parseArgs() -> Options {
    let args: Vec<_> = env::args_os().skip(1).collect();

    let mut parsed: Vec<CliArg> = Vec::new();
    let mut options = Options::default();

    for arg in args {
        if let Ok(arg_as_string) = arg.into_string() {
            let parsed_arg = match arg_as_string.as_str() {
                "-t" => CliArg {
                    raw: arg_as_string,
                    arg_type: CliArgType::Text,
                },
                "-l" => CliArg {
                    raw: arg_as_string,
                    arg_type: CliArgType::Limit,
                },
                "-p" => CliArg {
                    raw: arg_as_string,
                    arg_type: CliArgType::Path,
                },
                _ => CliArg {
                    raw: arg_as_string,
                    arg_type: CliArgType::Value,
                },
            };

            parsed.push(parsed_arg);
        }
    }

    for cli_value_pair in parsed.chunks(2) {
        let flag = cli_value_pair[0].arg_type;
        let value = cli_value_pair[1].clone();

        if matches!(flag, CliArgType::Value) || !matches!(value.arg_type, CliArgType::Value) {
            panic!("\nUsage typocalypse [OPTIONS]\n");
        };

        match flag {
            CliArgType::Path => options.path = Some(value.raw),
            CliArgType::Text => options.text = value.raw,
            CliArgType::Limit => options.limit = value.raw.parse().unwrap(),
            CliArgType::Value => {}
        }
    }

    options
}
