use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProjectKind {
    Anchor,
    Other,
}

pub(crate) fn detect_project_kind(root: &Path) -> ProjectKind {
    if root.join("Anchor.toml").exists() {
        ProjectKind::Anchor
    } else {
        ProjectKind::Other //maybe we will also add Shanked solana rust-native programs in the future
    }
}
