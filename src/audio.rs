use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use rodio::Source; 

pub struct AudioManager {
    bg_sink: Option<Sink>,
    bg_stream: Option<OutputStream>,
    fx_sink: Option<Sink>,
    fx_stream: Option<OutputStream>,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            bg_sink: None,
            bg_stream: None,
            fx_sink: None,
            fx_stream: None,
        }
    }

    pub fn play_running_loop(&mut self, sound_path: &str) {
        if self.fx_sink.is_some() {
            return;
        }
        if let Ok((stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(file) = File::open(sound_path) {
                let reader = BufReader::new(file);
                if let Ok(source) = Decoder::new(reader) {
                    if let Ok(sink) = Sink::try_new(&stream_handle) {
                        sink.set_volume(1.0);
                        sink.append(source.repeat_infinite());
                        sink.play();
                        self.fx_sink = Some(sink);
                        self.fx_stream = Some(stream);
                    }
                }
            }
        }
    }

    pub fn stop_running_loop(&mut self) {
        if let Some(sink) = &self.fx_sink {
            sink.stop();
        }
        self.fx_sink = None;
        self.fx_stream = None;
    }

    pub fn play_background_music(&mut self, music_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let file = File::open(music_path)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)?;
        let sink = Sink::try_new(&stream_handle)?;
        sink.set_volume(0.1);
        sink.append(source.repeat_infinite());
        sink.play();
        self.bg_sink = Some(sink);
        self.bg_stream = Some(stream);
        Ok(())
    }

    pub fn has_audio(&self) -> bool {
        self.bg_sink.is_some() || self.fx_sink.is_some()
    }
}