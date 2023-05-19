fn main() {
    glib_build_tools::compile_resources(
        &["data"],
        "./data/com.github.manenfu.Khazanah.gresource.xml",
        "resources.gresource",
    )
}
