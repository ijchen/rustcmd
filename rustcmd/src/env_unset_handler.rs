pub enum EnvUnsetHandler {
    Error,
    ReplaceWith(String),
    Panic,
}
