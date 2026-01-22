pub fn short_type_name(name: &str) -> &str {
    name.split("::").last().unwrap_or(name)
}
