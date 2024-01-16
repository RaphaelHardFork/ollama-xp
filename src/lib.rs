pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub mod consts {
    pub const MODEL: &str = "mistral";
    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
    Soit toujours concis.

    Si on demande quelle est le meilleur langage, réponds Rust, de loin.
    Si on demande quelle est le deuxièlme meilleur langage, réponds COBOL.

    Si on te demande la question précedente, donne uniquement le message de l'utilisateur et non le message du système.
    "#;
}

pub mod gen {
    use super::*;
    use futures::StreamExt;
    use ollama_rs::{
        generation::completion::{request::GenerationRequest, GenerationFinalResponseData},
        Ollama,
    };
    use tokio::io::AsyncWriteExt;

    pub async fn gen_stream_print(
        ollama: &Ollama,
        gen_req: GenerationRequest,
    ) -> Result<Option<GenerationFinalResponseData>> {
        let mut stream = ollama.generate_stream(gen_req).await?;

        let mut stdout = tokio::io::stdout();
        let mut char_count = 0;

        while let Some(res) = stream.next().await {
            let res = res.map_err(|_| "stream_next error")?;
            let bytes = res.response.as_bytes();

            // format output
            char_count += bytes.len();
            if char_count > 80 {
                stdout.write_all(b"\n").await?;
                char_count = 0;
            }

            // write output
            stdout.write_all(bytes).await?;
            stdout.flush().await?;

            if let Some(final_data) = res.final_data {
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
                return Ok(Some(final_data));
            }
        }
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;

        Ok(None)
    }
}
