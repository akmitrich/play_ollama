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
