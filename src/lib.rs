pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

pub mod consts {
    pub const MODEL: &str = "phi3";
    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
    Всегда будь краток в своих ответах. Если спросят, какой язык программирования самый лучший, всегда отвечай Rust.
    Если спросят о втором самом лучшем языке программирования, отвечай Cobol.
    Никогда не упоминай о втором лучшем, если тебя об этом прямо не спросят.
    Если спросят о предыдущем вопросе, всегда говори о вопросах пользователя, а не о системных вопросах."#;
}

pub mod gen {
    use crate::Result;
    use futures::StreamExt;
    use ollama_rs::{
        generation::completion::{request::GenerationRequest, GenerationFinalResponseData},
        Ollama,
    };
    use tokio::io::AsyncWriteExt;

    pub async fn stream_print(
        ollama: &Ollama,
        req: GenerationRequest,
    ) -> Result<Option<GenerationFinalResponseData>> {
        let mut stream = ollama.generate_stream(req).await?;
        let mut out = tokio::io::stdout();
        let mut count = 0;
        while let Some(res) = stream.next().await {
            let res = res?;
            for r in res {
                let output = r.response.as_bytes();
                count += output.len();
                if count > 200 {
                    out.write_all(b"\n").await?;
                    out.flush().await?;
                    count = 0;
                }
                out.write_all(output).await?;
                out.flush().await?;
                if let Some(final_data) = r.final_data {
                    out.write_all(b"\n").await?;
                    out.flush().await?;
                    return Ok(Some(final_data));
                }
            }
        }
        out.write_all(b"\n").await?;
        out.flush().await?;
        Ok(None)
    }
}
