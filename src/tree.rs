use std::ffi::OsStr;
use std::path::Path;

use ::ignore::WalkBuilder;
use serde::Serialize;

use crate::ignore::build_overrides;

#[derive(Debug, Clone)]
pub struct TreeOptions {
    pub max_depth: Option<usize>,
    pub all: bool,
    pub dirs_only: bool,
    pub no_gitignore: bool,
    pub ignore_patterns: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NodeKind {
    Directory,
    File,
}

#[derive(Debug, Clone, Serialize)]
pub struct TreeNode {
    pub name: String,
    pub kind: NodeKind,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<TreeNode>,
}

#[derive(Debug)]
pub struct BuiltTree {
    pub root: TreeNode,
    pub errors: Vec<String>,
}

pub fn build_tree(
    root: impl AsRef<Path>,
    options: &TreeOptions,
) -> Result<BuiltTree, ::ignore::Error> {
    let root = root.as_ref();
    let display_root = display_name(root);
    let root_path = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());

    let include_default_ignores = !options.all;
    let overrides = build_overrides(
        &root_path,
        include_default_ignores,
        &options.ignore_patterns,
    )?;

    let mut builder = WalkBuilder::new(&root_path);
    builder
        .hidden(!options.all)
        .parents(!options.all && !options.no_gitignore)
        .ignore(!options.all && !options.no_gitignore)
        .git_ignore(!options.all && !options.no_gitignore)
        .git_global(!options.all && !options.no_gitignore)
        .git_exclude(!options.all && !options.no_gitignore)
        .require_git(false)
        .follow_links(false)
        .same_file_system(false);

    if let Some(max_depth) = options.max_depth {
        builder.max_depth(Some(max_depth));
    }

    if let Some(overrides) = overrides {
        builder.overrides(overrides);
    }

    let mut errors = Vec::new();
    let mut root_node = TreeNode {
        name: display_root,
        kind: NodeKind::Directory,
        children: Vec::new(),
    };

    for entry in builder.build() {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path == root_path {
                    continue;
                }

                let kind = if entry
                    .file_type()
                    .is_some_and(|file_type| file_type.is_dir())
                {
                    NodeKind::Directory
                } else {
                    NodeKind::File
                };

                if options.dirs_only && kind != NodeKind::Directory {
                    continue;
                }

                if let Ok(relative) = path.strip_prefix(&root_path) {
                    insert_path(&mut root_node, relative, kind);
                }
            }
            Err(error) => errors.push(error.to_string()),
        }
    }

    sort_children(&mut root_node);

    Ok(BuiltTree {
        root: root_node,
        errors,
    })
}

fn insert_path(root: &mut TreeNode, relative: &Path, kind: NodeKind) {
    let mut node = root;
    let mut components = relative.components().peekable();

    while let Some(component) = components.next() {
        let is_last = components.peek().is_none();
        let name = component.as_os_str().to_string_lossy().into_owned();
        let child_kind = if is_last { kind } else { NodeKind::Directory };

        let child_index = match node.children.iter().position(|child| child.name == name) {
            Some(index) => {
                if is_last {
                    node.children[index].kind = child_kind;
                }
                index
            }
            None => {
                node.children.push(TreeNode {
                    name,
                    kind: child_kind,
                    children: Vec::new(),
                });
                node.children.len() - 1
            }
        };

        node = &mut node.children[child_index];
    }
}

fn sort_children(node: &mut TreeNode) {
    node.children.sort_by_cached_key(|child| {
        (
            matches!(child.kind, NodeKind::File),
            child.name.to_lowercase(),
        )
    });

    for child in &mut node.children {
        sort_children(child);
    }
}

fn display_name(path: &Path) -> String {
    if path == Path::new(".") {
        ".".to_string()
    } else {
        path.file_name()
            .unwrap_or_else(|| OsStr::new("."))
            .to_string_lossy()
            .into_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn sorts_directories_before_files() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("b.py"), "").unwrap();
        fs::create_dir(dir.path().join("a")).unwrap();

        let tree = build_tree(
            dir.path(),
            &TreeOptions {
                max_depth: None,
                all: false,
                dirs_only: false,
                no_gitignore: false,
                ignore_patterns: Vec::new(),
            },
        )
        .unwrap();

        assert_eq!(tree.root.children[0].name, "a");
        assert_eq!(tree.root.children[1].name, "b.py");
    }

    #[test]
    fn hides_default_noise() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("__pycache__")).unwrap();
        fs::write(dir.path().join("main.py"), "").unwrap();

        let tree = build_tree(
            dir.path(),
            &TreeOptions {
                max_depth: None,
                all: false,
                dirs_only: false,
                no_gitignore: false,
                ignore_patterns: Vec::new(),
            },
        )
        .unwrap();

        assert_eq!(tree.root.children.len(), 1);
        assert_eq!(tree.root.children[0].name, "main.py");
    }

    #[test]
    fn hides_dotfiles_by_default() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join(".env.example"), "").unwrap();
        fs::write(dir.path().join("main.py"), "").unwrap();

        let tree = build_tree(
            dir.path(),
            &TreeOptions {
                max_depth: None,
                all: false,
                dirs_only: false,
                no_gitignore: false,
                ignore_patterns: Vec::new(),
            },
        )
        .unwrap();

        assert_eq!(tree.root.children.len(), 1);
        assert_eq!(tree.root.children[0].name, "main.py");
    }
}
