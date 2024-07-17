mod token_output_stream;
mod utils;

#[macro_use]
extern crate tracing;

use tokio::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time;
use tokenizers::Tokenizer;

use candle_core::quantized::gguf_file;
use candle_core::{cuda, quantized};
use candle_core::{Device, Tensor};
use candle_transformers::generation::LogitsProcessor;

use anyhow::Result;
use candle_core::utils::cuda_is_available;
use candle_transformers::models::quantized_llama::ModelWeights;
use candle_transformers::utils::apply_repeat_penalty;
use hf_hub::api::tokio::Api;
use hf_hub::{Repo, RepoType};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use token_output_stream::TokenOutputStream;

use self::utils::format_size;

#[derive(Debug, Clone)]
pub struct Args {
    sample_len: usize,
    temperature: f64,
    seed: u64,
    repeat_penalty: f32,
    repeat_last_n: usize,
    gqa: usize,
    device: Device,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            sample_len: 1000,
            temperature: 0.8,
            seed: 299792458,
            repeat_penalty: 1.1,
            repeat_last_n: 64,
            gqa: 8,
            device: Device::new_cuda(0).unwrap_or(Device::Cpu),
        }
    }
}

impl Args {
    async fn tokenizer(&self) -> Result<Tokenizer> {
        let api = Api::new()?;

        let tokenizer_path = api.model("openchat/openchat_3.5".to_string())
            .get("tokenizer.json").await?;

        Tokenizer::from_file(tokenizer_path).map_err(anyhow::Error::msg)
    }

    async fn model(&self) -> Result<PathBuf> {
        let (repo, filename) = ("TheBloke/openchat_3.5-GGUF", "openchat_3.5.Q2_K.gguf");

        let api = Api::new()?;

        Ok(api.model(repo.to_string())
            .get(filename).await?)
    }
}


// 每次对话只有一个bot
pub struct ChatBot {
    // 最多2个线程访问
    model: Arc<Mutex<ModelWeights>>,
    tos: Arc<Mutex<TokenOutputStream>>,
    logits_processor: Arc<Mutex<LogitsProcessor>>,

    args: Args,
    eos_token: u32,
}

impl ChatBot {
    pub async fn from_args(args: Args) -> Result<Self> {
        let model_path = args.model().await?;

        let mut file = File::open(&model_path).await?.into_std().await;
        let start = time::Instant::now();

        // This is the model instance
        let model = gguf_file::Content::read(&mut file)?;
        let mut total_size_in_bytes = 0;
        for (_, tensor) in model.tensor_infos.iter() {
            let elem_count = tensor.shape.elem_count();
            total_size_in_bytes +=
                elem_count * tensor.ggml_dtype.type_size() / tensor.ggml_dtype.block_size();
        }
        info!("loaded {:?} tensors ({}) in {:.2}s",
            model.tensor_infos.len(),
            &format_size(total_size_in_bytes),
            start.elapsed().as_secs_f32(),
        );
        let model = ModelWeights::from_gguf(model, &mut file, &args.device)?;

        let tokenizer = args.tokenizer().await?;
        let tos = TokenOutputStream::new(tokenizer);

        let eos_token = tos.tokenizer().token_to_id("<|end_of_turn|>").unwrap();

        let logits_processor = LogitsProcessor::new(args.seed, Some(args.temperature), None);

        Ok(Self {
            model: Arc::new(Mutex::new(model)),
            tos: Arc::new(Mutex::new(tos)),
            logits_processor: Arc::new(Mutex::new(logits_processor)),
            args,
            eos_token,
        })
    }

    pub async fn from_default_args() -> Result<Self> {
        Self::from_args(Args::default()).await
    }

    /// 主线程用户 子线程bot
    pub fn chat(&mut self, input: String) -> Receiver<String> {
        let (tx, rx) = mpsc::channel(self.args.sample_len);

        let model = self.model.clone();
        let tos = self.tos.clone();
        let logits_processor = self.logits_processor.clone();
        let args = self.args.clone();
        let eos_token = self.eos_token;

        tokio::spawn(async move {
            let prompt = format!("User: {} <|end_of_turn|> Assistant: ", input.trim());

            let tokens = tos.lock().unwrap()
                .tokenizer()
                .encode(prompt, true).unwrap();

            let prompt_tokens = tokens.get_ids();
            let mut all_tokens = vec![];

            let start_prompt_processing = time::Instant::now();

            let mut next_token = {
                let input = Tensor::new(prompt_tokens, &args.device).unwrap()
                    .unsqueeze(0).unwrap();
                let logits = model.lock().unwrap()
                    .forward(&input, 0).unwrap();
                let logits = logits.squeeze(0).unwrap();
                logits_processor.lock().unwrap()
                    .sample(&logits).unwrap()
            };
            let prompt_dt = start_prompt_processing.elapsed();

            // 第一个单词
            let first_word = tos.lock().unwrap().next_token(next_token).unwrap();
            if let Some(t) = first_word {
                tx.send(t).await.unwrap();
            }

            let start_post_prompt = time::Instant::now();
            let to_sample = args.sample_len.saturating_sub(1);
            let mut sampled = 1;
            while sampled < to_sample {
                let input = Tensor::new(&[next_token], &args.device).unwrap()
                    .unsqueeze(0).unwrap();
                let logits = model.lock().unwrap()
                    .forward(&input, prompt_tokens.len() + sampled).unwrap();
                let logits = logits.squeeze(0).unwrap();
                let logits = if args.repeat_penalty == 1. {
                    logits
                } else {
                    let start_at = all_tokens.len().saturating_sub(args.repeat_last_n);
                    apply_repeat_penalty(
                        &logits,
                        args.repeat_penalty,
                        &all_tokens[start_at..],
                    ).unwrap()
                };
                next_token = logits_processor.lock().unwrap().sample(&logits).unwrap();
                all_tokens.push(next_token);
                let word = tos.lock().unwrap()
                    .next_token(next_token).unwrap();

                if let Some(t) = word {
                    tx.send(t).await.unwrap();
                }

                sampled += 1;
                if next_token == eos_token {
                    break;
                };
            }
            let rest = tos.lock().unwrap().decode_rest().unwrap();
            if let Some(rest) = rest {
                tx.send(rest).await.unwrap();
            }

            let dt = start_post_prompt.elapsed();

            info!("{} prompt tokens processed: {:.2} token/s",
            prompt_tokens.len(),
            prompt_tokens.len() as f64 / prompt_dt.as_secs_f64(),
        );
            info!("{sampled} tokens generated: {:.2} token/s",
            sampled as f64 / dt.as_secs_f64(),
        );
        });

        rx
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chatbot() -> Result<()> {
        tracing_subscriber::fmt::init();

        let mut chatbot = ChatBot::from_default_args().await?;
        let mut rx = chatbot.chat("你好".to_string());

        print!("Assistant: ");
        while let Some(word) = rx.recv().await {
            print!("{}", word);
        }

        Ok(())
    }
}
