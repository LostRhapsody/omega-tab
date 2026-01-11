fn main() {
    // Embed Windows icon resource
    #[cfg(windows)]
    {
        embed_resource::compile("assets/icon.rc", embed_resource::NONE);
    }
}
