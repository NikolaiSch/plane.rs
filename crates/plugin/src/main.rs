use libloading::Library;

unsafe fn main() -> anyhow::Result<()> {
    let library = Library::new("/usr/lib/ignition")?;

    todo!()
}
