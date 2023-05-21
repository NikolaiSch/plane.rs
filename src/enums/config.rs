pub enum ServerOpts {
    Host(&'static str),
    Port(u16),
    Subdirectory(Option<&'static str>)
}
