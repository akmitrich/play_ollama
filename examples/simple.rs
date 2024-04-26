use futures::StreamExt;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use play_ollama::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    Result,
};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();
    let model = MODEL.to_owned();
    let prompt = "Какой язык программирования самый лучший?".to_owned();
    let req = GenerationRequest::new(model, prompt).system(DEFAULT_SYSTEM_MOCK.to_owned());
    // let res = ollama.generate(req).await?;
    // println!("->> res={:?}", res);

    stream_print(&ollama, req).await?;
    Ok(())
}

async fn stream_print(ollama: &Ollama, req: GenerationRequest) -> Result<()> {
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
        }
    }
    out.write_all(b"\n").await?;

    Ok(())
}
