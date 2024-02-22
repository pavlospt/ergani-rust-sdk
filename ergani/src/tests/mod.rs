use regex::Regex;
use std::path::Path;

#[cfg(test)]
pub fn load_fixture_as_text(fixture_name: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(manifest_dir)
        .join("resources/tests/")
        .join(fixture_name);
    let regex = Regex::new(r"\n\s*").unwrap();
    let fixture_text = std::fs::read_to_string(path)
        .unwrap()
        .replace("\": ", "\":"); // Hacky way to convert multi-line JSON to single-line
    regex.replace_all(&fixture_text, "").to_string()
}
