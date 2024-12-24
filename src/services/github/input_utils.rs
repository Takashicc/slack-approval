use anyhow::Result;

const INPUT_PREFIX: &str = "INPUT_";

struct InputOptions {
    pub required: bool,
    pub trim_whitespace: bool,
}

fn get_input(name: &str, options: &InputOptions) -> Result<Option<String>> {
    let key = format!("{}{}", INPUT_PREFIX, name.replace(" ", "_").to_uppercase());
    let v = std::env::var(&key).ok();

    let v = if options.trim_whitespace {
        v.map(|v| v.trim().into())
    } else {
        v
    };

    if options.required {
        if v.is_none() {
            return Err(anyhow::anyhow!("Input '{}' is required", name));
        } else if v.as_ref().unwrap().is_empty() {
            return Err(anyhow::anyhow!("Input '{}' cannot be empty", name));
        }
    }

    Ok(v)
}

fn get_list_input(name: &str, options: &InputOptions) -> Result<Option<Vec<String>>> {
    get_input(name, options).map(|o| o.map(|v| v.split(',').map(|s| s.trim().into()).collect()))
}

pub fn get_optional_input(name: &str) -> Result<Option<String>> {
    get_input(
        name,
        &InputOptions {
            required: false,
            trim_whitespace: true,
        },
    )
}

pub fn get_required_input(name: &str) -> Result<String> {
    match get_input(
        name,
        &InputOptions {
            required: true,
            trim_whitespace: true,
        },
    )? {
        Some(v) => Ok(v),
        None => unreachable!(),
    }
}

pub fn get_optional_list_input(name: &str) -> Result<Option<Vec<String>>> {
    get_list_input(
        name,
        &InputOptions {
            required: false,
            trim_whitespace: true,
        },
    )
}

pub fn get_required_list_input(name: &str) -> Result<Vec<String>> {
    match get_list_input(
        name,
        &InputOptions {
            required: true,
            trim_whitespace: true,
        },
    )? {
        Some(v) => Ok(v),
        None => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        "required but not set",
        None,
        InputOptions {
            required: true,
            trim_whitespace: true,
        },
        Err("Input 'required but not set' is required".into())
    )]
    #[case(
        "required but empty",
        Some(""),
        InputOptions {
            required: true,
            trim_whitespace: true,
        },
        Err("Input 'required but empty' cannot be empty".into())
    )]
    #[case(
        "required and trimmed",
        Some(" va lue  "),
        InputOptions {
            required: true,
            trim_whitespace: true,
        },
        Ok(Some("va lue".into()))
    )]
    #[case(
        "required but not trimmed",
        Some(" va lue  "),
        InputOptions {
            required: true,
            trim_whitespace: false,
        },
        Ok(Some(" va lue  ".into())))
    ]
    #[case(
        "optional but not set",
        None,
        InputOptions {
            required: false,
            trim_whitespace: false,
        },
        Ok(None)
    )]
    #[case(
        "optional but empty",
        Some(""),
        InputOptions {
            required: false,
            trim_whitespace: false,
        },
        Ok(Some("".into()))
    )]
    #[case(
        "optional and trimmed",
        Some(" va lue  "),
        InputOptions {
            required: false,
            trim_whitespace: true,
        },
        Ok(Some("va lue".into()))
    )]
    #[case(
        "optional but not trimmed",
        Some(" va lue  "),
        InputOptions {
            required: false,
            trim_whitespace: false,
        },
        Ok(Some(" va lue  ".into())))
    ]
    fn test_get_input(
        #[case] name: &str,
        #[case] env_value: Option<&str>,
        #[case] options: InputOptions,
        #[case] expected: std::result::Result<Option<String>, String>,
    ) {
        let env_key = format!("{}{}", INPUT_PREFIX, name.replace(" ", "_").to_uppercase());
        std::env::remove_var(&env_key);

        if let Some(val) = env_value {
            std::env::set_var(&env_key, val);
        }

        let actual = get_input(name, &options);
        let actual = actual.map_err(|e| e.to_string());

        assert_eq!(actual, expected);
    }

    // TODO Add tests
}
