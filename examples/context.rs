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
    let prompts = [
        "Почему небо красного цвета?".to_owned(),
        "Какой вопрос я задал в начале?".to_owned(),
    ];
    let mut last_ctx = None;
    for prompt in prompts {
        let mut req =
            GenerationRequest::new(model.to_owned(), prompt).system(DEFAULT_SYSTEM_MOCK.to_owned());
        if let Some(ctx) = last_ctx.take() {
            req = req.context(ctx);
        }
        last_ctx = stream_print(&ollama, req).await?.map(|data| data.context);
    }

    Ok(())
}
