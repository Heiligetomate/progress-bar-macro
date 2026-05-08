pub const LABEL_NAME: &str = "max_value";
pub const ONLY_STRUCTS: &str = "#[derive(ProgressBar)] only supports structs";
pub const ONLY_NAMED: &str = "#[derive(ProgressBar)] only supports structs with named fields";
pub const NO_FIELDS: &str = "#[derive(ProgressBar)] requires exactly one field with #[max_value = \"...\"], found none";
pub const TOO_MANY_FIELDS: &str = "[max_value] used on multiple fields — exactly one allowed";
pub const PARSE_ERROR: &str = "Can not parse this shit";
