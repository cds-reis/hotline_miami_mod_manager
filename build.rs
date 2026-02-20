use std::io;

fn main() -> io::Result<()> {
    #[cfg(windows)]
    {
        winres::WindowsResource::new()
            .set_icon("assets/icons/icon.ico")
            .compile()?;
    }
    Ok(())
}
