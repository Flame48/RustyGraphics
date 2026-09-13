fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/logo.ico");
        res.set("FileDescription", "Graphics Viewer");
        res.set("ProductName", "Graphics Viewer");
        res.set("OriginalFilename", "graphics-view.exe");
        res.set("CompanyName", "Advait Subhash Rajan");
        res.compile().expect("Failed to embed icon resource");
    }
}
