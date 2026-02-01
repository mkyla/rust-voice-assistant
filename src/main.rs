use std::sync::mpsc;
use anyhow::{Result, anyhow};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Stream};
use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams};
use elevenlabs::client::ElevenLabsClient;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok(); // if using dotenv

    let api_key = std::env::var("ELEVENLABS_API_KEY").map_err(|_| anyhow!("ELEVENLABS_API_KEY not set"))?;
    let client = ElevenLabsClient::new(api_key);

    // Assume model path, in real, download or set
    let model_path = "models/ggml-base.en.bin"; // need to provide
    let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())?;

    let host = cpal::default_host();
    let device = host.default_input_device().ok_or(anyhow!("No input device"))?;
    let config = device.default_input_config()?;
    let sample_rate = config.sample_rate().0;

    let (tx, rx) = mpsc::channel::<Vec<f32>>();

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &_| {
            let _ = tx.send(data.to_vec());
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )?;
    stream.play()?;

    let (_stream_out, stream_handle) = OutputStream::try_default()?;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let mut audio_data = Vec::new();
        while let Ok(chunk) = rx.try_recv() {
            audio_data.extend(chunk);
        }

        if audio_data.is_empty() {
            continue;
        }

        // Assume 16khz mono f32, resample if needed
        let mut params = FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_translate(false);
        params.set_no_context(true);
        params.set_single_segment(true);

        let mut state = ctx.create_state()?;
        state.full(params, &audio_data)?;
        let num_segments = state.full_n_segments()?;
        let mut text = String::new();
        for i in 0..num_segments {
            text.push_str(&state.full_get_segment_text(i)?);
        }

        println!("Transcribed: {}", text);

        let response = process_command(&text);

        let tts_bytes = client.generate_speech("21m00Tcm4TlvDq8ikWAM", &response).await?; // example voice id

        let cursor = Cursor::new(tts_bytes);
        let source = Decoder::new(cursor)?;
        let sink = Sink::try_new(&stream_handle)?;
        sink.append(source);
        sink.sleep_until_end();
    }
}

fn process_command(text: &str) -> String {
    if text.to_lowercase().contains("hello") {
        "Hello! How can I help?".to_string()
    } else if text.to_lowercase().contains("time") {
        format!("The current time is {}", chrono::Utc::now().format("%H:%M:%S"))
    } else {
        "Sorry, I didn't understand.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_command_hello() {
        assert_eq!(process_command("hello"), "Hello! How can I help?");
    }

    #[test]
    fn test_process_command_unknown() {
        assert_eq!(process_command("unknown"), "Sorry, I didn't understand.");
    }
}