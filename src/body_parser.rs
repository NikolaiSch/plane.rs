use {
    crate::Res,
    std::io::Write
};

pub trait ToHTTP {
    fn write_to_stream(&mut self, mut stream: impl Write) -> anyhow::Result<()> {
        let text = self.get_text()?;

        for line in text {
            writeln!(stream, "{}\n", line)?
        }
        Ok(())
    }

    fn get_text(&mut self) -> anyhow::Result<Vec<String>>;
}

impl ToHTTP for Res {
    fn get_text(&mut self) -> anyhow::Result<Vec<String>> {
        let mut v = vec![];
        v.push(format!("{} {} {}", "HTTP/1.1", self.status(), "OK"));

        let mut headers: Vec<_> = self
            .headers()
            .into_iter()
            .map(|(name, value)| format!("{}: {}", name, value.to_str().unwrap()))
            .collect();

        v.append(&mut headers);
        v.append(self.body_mut());

        Ok(v)
    }
}
