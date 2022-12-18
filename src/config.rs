pub const REPO: &str = "https://gitlab.gnome.org/World/Rust/gtk-rust-template.git";
pub const TEMPLATE_DIR: &str = "gtk-rust-template";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_repo() {
        assert_eq!(
            REPO,
            "https://gitlab.gnome.org/World/Rust/gtk-rust-template.git"
        );
    }

    #[test]
    #[should_panic]
    fn it_not_should_empty() {
        assert_eq!(REPO, "");
    }

    #[test]
    #[should_panic]
    fn it_not_should_blank() {
        assert_eq!(REPO, " ");
    }

    #[test]
    fn test_const_repo_not_equal() {
        assert_ne!(REPO, "fake_url");
    }

    #[test]
    fn test_const_template_dir() {
        assert_eq!(TEMPLATE_DIR, "gtk-rust-template");
    }

    #[test]
    #[should_panic]
    fn template_dir_not_should_empty() {
        assert_eq!(TEMPLATE_DIR, "");
    }

    #[test]
    #[should_panic]
    fn template_dir_not_should_be_blank() {
        assert_eq!(TEMPLATE_DIR, " ");
    }

    #[test]
    fn test_const_template_dir_not_equal() {
        assert_ne!(TEMPLATE_DIR, "fake_dir");
    }
}
