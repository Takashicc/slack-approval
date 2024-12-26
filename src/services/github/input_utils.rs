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

pub fn get_list_input(name: &str) -> Result<Vec<String>> {
    get_input(
        name,
        &InputOptions {
            required: false,
            trim_whitespace: true,
        },
    )
    .map(|o| {
        o.map(|v| {
            v.split(',')
                .map(|s| s.trim().into())
                .filter(|s: &String| !s.is_empty())
                .collect::<Vec<String>>()
        })
        .unwrap_or_else(|| vec![])
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::*;

    fn initialize_env_variable(name: &str, env_value: Option<&str>) {
        let env_key = format!("{}{}", INPUT_PREFIX, name.replace(" ", "_").to_uppercase());
        std::env::remove_var(&env_key);

        if let Some(val) = env_value {
            std::env::set_var(&env_key, val);
        }
    }

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
        #[case] expected: Result<Option<String>, String>,
    ) {
        initialize_env_variable(name, env_value);

        let actual = get_input(name, &options);
        let actual = actual.map_err(|e| e.to_string());

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(
        "required but not set",
        None,
        Ok(vec![])
    )]
    #[case(
        "required but empty",
        Some(""),
        Ok(vec![])
    )]
    #[case(
        "values",
        Some("v1, v2, v3"),
        Ok(vec!["v1".into(), "v2".into(), "v3".into()])
    )]
    #[case(
        "values with empty",
        Some("v1, , v3"),
        Ok(vec!["v1".into(), "v3".into()])
    )]
    fn test_get_list_input(
        #[case] name: &str,
        #[case] env_value: Option<&str>,
        #[case] expected: Result<Vec<String>, String>,
    ) {
        initialize_env_variable(name, env_value);

        let actual = get_list_input(name);
        let actual = actual.map_err(|e| e.to_string());

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("none", None, None)]
    #[case("empty", Some(""), Some("".into()))]
    #[case("value", Some("value"), Some("value".into()))]
    fn test_get_optional_input(
        #[case] name: &str,
        #[case] env_value: Option<&str>,
        #[case] expected: Option<String>,
    ) {
        initialize_env_variable(name, env_value);

        let actual = get_optional_input(name).unwrap();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("none", None, Err("Input 'none' is required".into()))]
    #[case("empty", Some(""), Err("Input 'empty' cannot be empty".into()))]
    #[case("value", Some("value"), Ok("value".into()))]
    fn test_get_required_input(
        #[case] name: &str,
        #[case] env_value: Option<&str>,
        #[case] expected: Result<String, String>,
    ) {
        initialize_env_variable(name, env_value);

        let actual = get_required_input(name).map_err(|e| e.to_string());
        assert_eq!(actual, expected);
    }
}
