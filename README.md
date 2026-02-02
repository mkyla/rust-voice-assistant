# Rust Voice Assistant

A lightweight Rust-based voice assistant with speech-to-text and text-to-speech capabilities, using Whisper for STT and ElevenLabs for TTS.

## 📖 Overview

Rust Voice Assistant captures audio asynchronously, transcribes it with Whisper, processes basic commands, and responds with ElevenLabs TTS. Ideal for offline STT and cloud TTS integration.

## 🎬 Demo

Set API key, run `cargo run`, say "time", hear response.

## ✨ Features

### 🎤 STT & TTS
- Whisper for transcription.
- ElevenLabs for speech synthesis.

### 🚀 Async
- Tokio for concurrency.
- CPAL for audio capture.

### 🛠️ Command Processing
- Basic NLP for commands.

## 📦 Installation

### 🔧 Compile from Source

# Clone
git clone https://github.com/mkyla/rust-voice-assistant.git
cd rust-voice-assistant

# Build
cargo build --release

## 📋 Usage Guide

1. Set ELEVENLABS_API_KEY
2. Download Whisper model to models/
3. Run `./target/release/rust-voice-assistant`

Say commands, get responses.

## ⚙️ Configuration

- API Key: ELEVENLABS_API_KEY env var
- Model: models/ggml-base.en.bin

## 🛠️ Development

### 📁 Project Structure

rust-voice-assistant/
├── src/main.rs
├── Cargo.toml
├── .github/workflows/ci.yml
└── README.md

### 🧩 Core Components

1. **main.rs**: Audio loop, transcription, command handling, TTS.

### 🛠️ Tech Stack

- Rust 2021
- STT: Whisper
- TTS: ElevenLabs API
- Audio: CPAL, Rodio

## 📄 License

BSD-3-Clause