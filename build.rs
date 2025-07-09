fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "my_gtk_app.gresource",
    );
}

// for automating compiling schema
// fn compile_schema() {
//     let schema_dir = std::path::Path::new("/src/org.my_gtk_app.MyGtkApp.gschema.xml");

//     std::fs::create_dir_all(&schema_dir).expect("Failed to create schema dir");

//     std::process::Command::new("glib-compile-schemas")
//         .arg(schema_dir)
//         .status()
//         .expect("Failed to compile schemas");

//     println!(
//         "cargo:rustc-env-GSETTINGS_SCHEMA_DIR={}",
//         schema_dir.display()
//     );
// }
