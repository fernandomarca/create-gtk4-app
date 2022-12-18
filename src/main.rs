#[allow(unused_variables, dead_code)]
use serde_derive::Deserialize;
use std::fs;
use std::process::{exit, Command, Stdio};

fn main() {
    const REPO: &str = "https://gitlab.gnome.org/World/Rust/gtk-rust-template.git";
    const TEMPLATE_DIR: &str = "gtk-rust-template";

    start();
    fn start() {
        get_repository();
        create_project();

        let project_name = folder_project_name();
        match project_name {
            Some(value) => {
                move_project_folder(value.as_str());
                cleanup();
            }
            None => return,
        }
    }

    fn get_repository() {
        let gtk_rust_template_clone = Command::new("git")
            .args(["clone", "-q", REPO])
            .spawn()
            .unwrap();

        let _output = gtk_rust_template_clone
            .wait_with_output()
            .expect("get repository error!");
    }
    fn create_project() {
        //create project
        let create_project = Command::new("python3")
            .arg("create-project.py")
            .current_dir(TEMPLATE_DIR)
            .spawn();

        if let Ok(child) = create_project {
            let _output = child
                .wait_with_output()
                .expect("create_project error!")
                .stdout;
        };
    }
    fn folder_project_name() -> Option<String> {
        //create project
        let folder_list = Command::new("ls")
            .arg("-lt")
            .current_dir(TEMPLATE_DIR)
            .stdout(Stdio::piped())
            .spawn();

        match folder_list {
            Ok(child) => {
                let output = child
                    .wait_with_output()
                    .expect("older_project_name error!")
                    .stdout;
                let transform_string = String::from_utf8(output).expect("read from_utf8");
                let list: Vec<String> = transform_string
                    .split("\n")
                    .map(|item| item.to_string())
                    .collect();

                let mut folder_ls_info: Vec<String> = list
                    .get(1)
                    .map(|block| block.split(" ").map(|item| item.to_string()))
                    .unwrap()
                    .collect();
                folder_ls_info.pop()
            }
            _ => None,
        }
    }
    fn move_project_folder(origin: &str) {
        let move_command = Command::new("mv")
            .args(["-f", origin, "../"])
            .current_dir(TEMPLATE_DIR)
            .spawn();

        if let Ok(child) = move_command {
            let _output = child.wait_with_output().expect("move_command error");
        };
    }
    fn cleanup() {
        let cleanup = Command::new("rm").args(["-rf", TEMPLATE_DIR]).spawn();

        if let Ok(child) = cleanup {
            let _output = child.wait_with_output().expect("aguardando");
        };
    }
    fn _read_toml_file() {
        #[allow(dead_code)]
        #[derive(Deserialize)]
        struct Data {
            package: Package,
        }
        #[allow(dead_code)]
        #[derive(Deserialize)]
        struct Package {
            name: String,
        }

        let filename = "./gtk-rust-template/Cargo.toml";
        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file `{}`", filename);
                exit(1);
            }
        };

        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from `{}`", filename);
                exit(1);
            }
        };
    }
}
