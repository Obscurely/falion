fn main() {
    #[cfg(windows)]
    {
        embed_resource::compile("resources\\windows\\resources.rc", embed_resource::NONE);
    }
    slint_build::compile("./ui/main.slint").unwrap();
}
