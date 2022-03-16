use redacted::*;

#[test]
fn test_api() {
    #[allow(dead_code)]
    #[derive(Redact)]
    struct Sample {
        username: &'static str,
        email: &'static str,
        #[redacted]
        password: &'static str,
    }

    let instance = Sample {
        username: "FoseFx",
        email: "max@bmn.dev",
        password: "hunter2",
    };

    let output = format!("{:?}", instance);
    let expected =
        "Sample { username: \"FoseFx\", email: \"max@bmn.dev\", password: \"<redacted>\" }";

    assert_eq!(output, expected);
}

#[test]
fn test_api_2() {
    #[allow(dead_code)]
    #[derive(Redact)]
    struct Sample {
        username: &'static str,
        #[redacted]
        password: &'static str,
        email: &'static str,
    }

    let instance = Sample {
        username: "FoseFx",
        email: "max@bmn.dev",
        password: "hunter2",
    };

    let output = format!("{:#?}", instance);
    let expected = "Sample {\n    username: \"FoseFx\",\n    password: \"<redacted>\",\n    email: \"max@bmn.dev\",\n}";

    assert_eq!(output, expected);
}
