use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use xp_ollama::Result;
use xp_ollama::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    gen::gen_stream_print,
};

#[tokio::main]
async fn main() -> Result<()> {
    // by default localhost:11434
    let ollama = Ollama::default();
    let model = MODEL.to_string();
    let prompt = "Quelle est le meilleur langage de programmation".to_string();

    let gen_req = GenerationRequest::new(model, prompt).system(DEFAULT_SYSTEM_MOCK.to_string());

    // single res gen
    // let res = ollama.generate(gen_req).await?;
    // println!("->> res: {}", res.response);

    // stream res gen
    gen_stream_print(&ollama, gen_req).await?;

    Ok(())
}
