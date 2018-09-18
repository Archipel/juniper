#[macro_export]
macro_rules! graphql_stringscalar {

    // Entry point:
    // RustName as "GraphQLName" { ... }
    ($name:ty as $outname:tt) => {
        graphql_scalar!($name as $outname {
            description: (&format!("Rust type {} as {}", stringify!($name), $outname)[..])

            resolve(&self) -> Value {
                Value::string(self.to_string())
            }

            from_input_value(v: &InputValue) -> Option<$name> {
                // If there's a parse error here, simply return None. Juniper will
                // present an error to the client.
                match v.as_string_value() {
                    Some(i) => {
                        match i.parse::<$name>() {
                            Ok(val) => Some(val),
                            Err(_) => None
                        }
                    },
                    None => None
                }
            }
        });
    };

    // Entry point:
    // RustName { ... }
    ($name:ty) => {
        graphql_stringscalar!($name as (stringify!($name)));
    };
}
