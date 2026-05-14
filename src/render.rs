use anstyle::{AnsiColor, Style};

use crate::tree::{NodeKind, TreeNode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeStyle {
    Unicode,
    Ascii,
}

impl TreeStyle {
    fn branch(self, is_last: bool) -> &'static str {
        match (self, is_last) {
            (Self::Unicode, true) => "└── ",
            (Self::Unicode, false) => "├── ",
            (Self::Ascii, true) => "`-- ",
            (Self::Ascii, false) => "|-- ",
        }
    }

    fn prefix(self, is_last: bool) -> &'static str {
        match (self, is_last) {
            (Self::Unicode, true) => "    ",
            (Self::Unicode, false) => "│   ",
            (Self::Ascii, true) => "    ",
            (Self::Ascii, false) => "|   ",
        }
    }
}

pub fn render_tree(root: &TreeNode, use_color: bool, style: TreeStyle) -> String {
    let mut output = String::new();
    output.push_str(&format_name(root, use_color));
    output.push('\n');

    for (index, child) in root.children.iter().enumerate() {
        render_child(
            child,
            "",
            index + 1 == root.children.len(),
            use_color,
            style,
            &mut output,
        );
    }

    let counts = count_children(root);
    output.push('\n');
    output.push_str(&format!(
        "{} {}, {} {}\n",
        counts.directories,
        pluralize(counts.directories, "directory", "directories"),
        counts.files,
        pluralize(counts.files, "file", "files")
    ));

    output
}

pub fn render_json(root: &TreeNode) -> serde_json::Result<String> {
    serde_json::to_string_pretty(root)
}

fn render_child(
    node: &TreeNode,
    prefix: &str,
    is_last: bool,
    use_color: bool,
    style: TreeStyle,
    output: &mut String,
) {
    output.push_str(prefix);
    output.push_str(style.branch(is_last));
    output.push_str(&format_name(node, use_color));
    output.push('\n');

    let next_prefix = format!("{prefix}{}", style.prefix(is_last));

    for (index, child) in node.children.iter().enumerate() {
        render_child(
            child,
            &next_prefix,
            index + 1 == node.children.len(),
            use_color,
            style,
            output,
        );
    }
}

fn format_name(node: &TreeNode, use_color: bool) -> String {
    if !use_color || !matches!(node.kind, NodeKind::Directory) {
        return node.name.clone();
    }

    let style = Style::new().fg_color(Some(AnsiColor::Blue.into())).bold();
    format!("{style}{}{style:#}", node.name)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct TreeCounts {
    directories: usize,
    files: usize,
}

fn count_children(root: &TreeNode) -> TreeCounts {
    root.children
        .iter()
        .fold(TreeCounts::default(), |mut counts, child| {
            counts.add_node(child);
            counts
        })
}

impl TreeCounts {
    fn add_node(&mut self, node: &TreeNode) {
        match node.kind {
            NodeKind::Directory => self.directories += 1,
            NodeKind::File => self.files += 1,
        }

        for child in &node.children {
            self.add_node(child);
        }
    }
}

fn pluralize(count: usize, singular: &'static str, plural: &'static str) -> &'static str {
    if count == 1 { singular } else { plural }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_tree_connectors() {
        let root = TreeNode {
            name: ".".to_string(),
            kind: NodeKind::Directory,
            children: vec![TreeNode {
                name: "main.py".to_string(),
                kind: NodeKind::File,
                children: Vec::new(),
            }],
        };

        assert_eq!(
            render_tree(&root, false, TreeStyle::Unicode),
            ".\n└── main.py\n\n0 directories, 1 file\n"
        );
    }

    #[test]
    fn renders_ascii_tree_connectors() {
        let root = TreeNode {
            name: ".".to_string(),
            kind: NodeKind::Directory,
            children: vec![TreeNode {
                name: "main.py".to_string(),
                kind: NodeKind::File,
                children: Vec::new(),
            }],
        };

        assert_eq!(
            render_tree(&root, false, TreeStyle::Ascii),
            ".\n`-- main.py\n\n0 directories, 1 file\n"
        );
    }

    #[test]
    fn counts_rendered_nodes_without_root() {
        let root = TreeNode {
            name: ".".to_string(),
            kind: NodeKind::Directory,
            children: vec![TreeNode {
                name: "pkg".to_string(),
                kind: NodeKind::Directory,
                children: vec![TreeNode {
                    name: "main.py".to_string(),
                    kind: NodeKind::File,
                    children: Vec::new(),
                }],
            }],
        };

        assert_eq!(
            render_tree(&root, false, TreeStyle::Unicode),
            ".\n└── pkg\n    └── main.py\n\n1 directory, 1 file\n"
        );
    }
}
