use std::{fs, path::Path};

use ollama_rs::Ollama;
use simple_fs::{ensure_dir, read_to_string, save_be_f64, save_json};
use tokio::{self, io::split};
use xp_ollama::{consts::MODEL, Result};

const MOCK_DIR: &str = "_mock-data";
const EMBEDDING_DIR: &str = ".embedding-data";

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    ensure_dir(EMBEDDING_DIR)?;

    let txt = read_to_string(Path::new(MOCK_DIR).join("embeddings.txt"))?;
    let splits = simple_text_splitter(&txt, 500)?;

    println!("->> splits count: {}", splits.len());

    for (i, seg) in splits.into_iter().enumerate() {
        let file_name = format!("embeddings-{:0>2}.txt", i);
        let path = Path::new(EMBEDDING_DIR).join(file_name);
        fs::write(path, &seg)?;

        println!("->> text length: {}", txt.len());
        let res = ollama
            .generate_embeddings(MODEL.to_string(), seg, None)
            .await?;

        println!("->> embeddings length: {}", res.embeddings.len());

        let file_name = format!("embeddings-{:0>2}.json", i);
        save_json(Path::new(EMBEDDING_DIR).join(file_name), &res.embeddings)?;

        let file_name = format!("embeddings-{:0>2}.be-f64.bin", i);
        save_be_f64(Path::new(EMBEDDING_DIR).join(file_name), &res.embeddings)?;
    }

    Ok(())
}

fn simple_text_splitter(txt: &str, num: u32) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let mut last = 0;
    let mut count = 0;

    for (idx, _) in txt.char_indices() {
        count += 1;
        if count == num {
            result.push(&txt[last..idx + 1]);
            last = idx + 1;
            count = 0;
        }
    }

    if last < txt.len() {
        result.push(&txt[last..]);
    }

    Ok(result.into_iter().map(String::from).collect())
}
