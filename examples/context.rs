use ollama_rs::generation::completion::GenerationContext;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use simple_fs::{ensure_file_dir, save_json};
use xp_ollama::Result;
use xp_ollama::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    gen::gen_stream_print,
};

#[tokio::main]
async fn main() -> Result<()> {
    // by default localhost:11434
    let ollama = Ollama::default();
    let prompts = &[
        "Pourquoi le ciel est rouge ? (soit concis)",
        "Quelle était ma dernière question ?",
    ];

    let mut last_ctx: Option<GenerationContext> = None;

    for prompt in prompts {
        println!("\n->> prompt: {}", prompt);
        let mut gen_req = GenerationRequest::new(MODEL.to_string(), prompt.to_string());

        if let Some(last_ctx) = last_ctx.take() {
            gen_req = gen_req.context(last_ctx);
        }

        let final_data = gen_stream_print(&ollama, gen_req).await?;

        if let Some(final_data) = final_data {
            last_ctx = Some(final_data.context);

            // save for debug
            let ctx_file_path = ".context-data/ctx.json";
            ensure_file_dir(ctx_file_path)?;
            save_json(ctx_file_path, &last_ctx)?;
        }
    }

    Ok(())
}
