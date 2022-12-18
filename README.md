## A simple program for initialize

## Rust + GTK + Meson + Flatpak application boilerplate template


This launcher was created to work with the official repository [https://gitlab.gnome.org/World/Rust/gtk-rust-template.git]()
and can suffer breakages with changes.


## Build and use.

1. cargo build --release
2. cp target/release/create-gtk4-rs-app /bin
3. execute create-gtk4-rs-app (the project will be created in the parent where it was executed).


## Building the project

Make sure you have `flatpak` and `flatpak-builder` installed. Then run the commands below. Replace `<application_id>` with the value you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```plaintext
flatpak install org.gnome.Sdk//43 org.freedesktop.Sdk.Extension.rust-stable//22.08 org.gnome.Platform//43
flatpak-builder --user flatpak_app build-aux/<application_id>.Devel.json
```

Running the project

Once the project is build, run the command below. Replace Replace `<application_id>` and `<project_name>` with the values you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```plaintext
flatpak-builder --run flatpak_app build-aux/<application_id>.Devel.json <project_name>
```


## Fetuares

* [ ] allow to create the template in a specified folder
* [ ] build a command line interface complete management
