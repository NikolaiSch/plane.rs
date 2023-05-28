use {
    crate::Response,
    tokio::{
        io::AsyncWriteExt,
        net::tcp::OwnedWriteHalf
    },
    tracing::*
};

pub trait ToHTTP {
    #[instrument(level = Level::DEBUG, name = "Writing the Response to the `OwnedWriteHalf`", skip_all )]
    async fn write_to_stream(&mut self, stream: &mut OwnedWriteHalf) -> anyhow::Result<()> {
        let text = self.get_text()?;
        event!(Level::TRACE, "successfully called get_text()");

        for line in text {
            event!(Level::DEBUG, line = line, i = 0);

            stream.writable().await?;
            stream.write_all(line.as_bytes()).await?;
        }
        Ok(())
    }
    fn get_text(&mut self) -> anyhow::Result<Vec<String>>;
}

impl ToHTTP for Response<Vec<String>> {
    fn get_text(&mut self) -> anyhow::Result<Vec<String>> {
        let mut v = vec![];
        v.push(format!("{} {} {}", "HTTP/1.1", self.status(), "OK"));

        let mut headers: Vec<_> = self
            .headers()
            .into_iter()
            .map(|(name, value)| format!("{}: {}", name, value.to_str().unwrap()))
            .collect();

        v.append(&mut headers);
        v.append(&mut self.body_mut().to_owned());

        Ok(v)
    }
}
