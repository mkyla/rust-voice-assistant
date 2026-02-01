# Rust Voice Assistant

A simple voice assistant built in Rust using ElevenLabs for TTS and Whisper for STT.

## Features

- Asynchronous audio capture with CPAL
- Speech-to-text with Whisper
- Text-to-speech with ElevenLabs
- Basic command processing
- Error handling
- Tests
- CI/CD with GitHub Actions

## Setup

1. Clone the repo
2. Download Whisper model (e.g., ggml-base.en.bin) to models/
3. Set ELEVENLABS_API_KEY environment variable
4. Run `cargo run`

## Usage

The assistant listens for 5 seconds, transcribes speech, processes commands, and responds via TTS.

Commands:
- "hello" -> Greets
- "time" -> Tells current time

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```