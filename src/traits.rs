
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tracing::Level;
use tracing::event;
use tracing::instrument;

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