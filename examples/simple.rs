use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use play_ollama::gen::stream_print;
use play_ollama::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();
    let model = MODEL.to_owned();
    let prompt = "Какой язык программирования самый лучший?".to_owned();
    let req = GenerationRequest::new(model, prompt).system(DEFAULT_SYSTEM_MOCK.to_owned());

    stream_print(&ollama, req).await?;
    Ok(())
}
