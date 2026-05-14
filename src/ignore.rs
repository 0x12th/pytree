use ::ignore::overrides::{Override, OverrideBuilder};
use std::path::Path;

pub const DEFAULT_IGNORE_PATTERNS: &[&str] = &[
    ".git/",
    ".venv/",
    "venv/",
    "__pycache__/",
    ".pytest_cache/",
    ".mypy_cache/",
    ".ruff_cache/",
    ".coverage",
    "htmlcov/",
    "dist/",
    "build/",
    "*.egg-info/",
    ".idea/",
    ".vscode/",
    "node_modules/",
];

pub fn build_overrides(
    root: &Path,
    include_defaults: bool,
    user_patterns: &[String],
) -> Result<Option<Override>, ignore::Error> {
    if !include_defaults && user_patterns.is_empty() {
        return Ok(None);
    }

    let mut builder = OverrideBuilder::new(root);

    if include_defaults {
        for pattern in DEFAULT_IGNORE_PATTERNS {
            builder.add(&format!("!{pattern}"))?;
        }
    }

    for pattern in user_patterns {
        builder.add(&format!("!{pattern}"))?;
    }

    Ok(Some(builder.build()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_patterns_include_common_python_noise() {
        assert!(DEFAULT_IGNORE_PATTERNS.contains(&"__pycache__/"));
        assert!(DEFAULT_IGNORE_PATTERNS.contains(&".ruff_cache/"));
        assert!(DEFAULT_IGNORE_PATTERNS.contains(&"*.egg-info/"));
    }
}
