use crate::judge::{case, check_eq};
use crate::user::Credentials;

fn creds(user: &str, password: &str, scopes: &[&str]) -> Credentials {
    Credentials {
        user: user.to_string(),
        password: password.to_string(),
        scopes: scopes.iter().map(|s| s.to_string()).collect(),
    }
}

pub fn run() {
    case("example-1", || {
        let c = creds("ann", "hunter2", &["read", "write"]);
        check_eq(
            format!("{c:?}"),
            r#"Credentials { user: "ann", password: <redacted>, scopes: ["read", "write"] }"#,
        )?;
        check_eq(
            format!("{c:#?}"),
            concat!(
                "Credentials {\n",
                "    user: \"ann\",\n",
                "    password: <redacted>,\n",
                "    scopes: [\n",
                "        \"read\",\n",
                "        \"write\",\n",
                "    ],\n",
                "}",
            ),
        )
    });
    case("case-1", || {
        // An empty password is redacted too; no scopes; the user name is escaped.
        let c = creds("\"root\"", "", &[]);
        check_eq(
            format!("{c:#?}"),
            concat!(
                "Credentials {\n",
                "    user: \"\\\"root\\\"\",\n",
                "    password: <redacted>,\n",
                "    scopes: [],\n",
                "}",
            ),
        )
    });
    case("case-2", || {
        // Inside another value: one more level of indentation.
        check_eq(
            format!("{:#?}", Some(creds("bo", "pw", &["admin"]))),
            concat!(
                "Some(\n",
                "    Credentials {\n",
                "        user: \"bo\",\n",
                "        password: <redacted>,\n",
                "        scopes: [\n",
                "            \"admin\",\n",
                "        ],\n",
                "    },\n",
                ")",
            ),
        )?;
        check_eq(
            format!("{:?}", [creds("a", "b", &[])]),
            r#"[Credentials { user: "a", password: <redacted>, scopes: [] }]"#,
        )
    });
}
