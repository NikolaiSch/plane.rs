use crate::traits::ToHTTP;

use {
    crate::Response,
    tokio::{
        io::AsyncWriteExt,
        net::tcp::OwnedWriteHalf
    },
    tracing::*
};



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
