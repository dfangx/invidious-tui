use mpv::{
    MpvHandler,
    MpvHandlerBuilder,
    Event,
};
use std::process::Command;
use std::os::unix::net::UnixStream;
use std::io::{
    Read,
    Write,
};
use failure::Error;
use serde::Deserialize;
use std::{
    thread,
    time::Duration,
    convert::TryInto,
};

#[derive(Deserialize, Debug)]
struct MpvResponse<T> {
    data: T,
    request_id: u32,
    error: String,
}

pub enum ActivePlayer {
    AudioPlayer,
    VideoPlayer,
    NoPlayer,
}

pub struct Player {
    audio: MpvHandler,
    video: Option<UnixStream>,
    active: ActivePlayer,
}

impl Player {
    pub fn track_changed(&mut self) -> bool {
        if let Some(event) = self.audio.wait_event(0.0) {
            if let Event::StartFile = event {
                return true
            }
        }
        false
    }
    
    pub fn get_percent_pos(&self) -> u16 {
        match self.audio.get_property::<i64>("percent-pos") {
            Ok(percent) => return percent.try_into().unwrap_or(0),
            Err(e) => log::error!("Unable to get percent position: {}", e),
        }
        0
    }
    
    pub fn play(&mut self, url: String, is_video: bool) {
        if is_video {
            if let ActivePlayer::AudioPlayer = self.active {
                self.pause_audio();
            }

            self.active = ActivePlayer::VideoPlayer;
            if self.video.is_some() {
                self.stop();
            }
            let args = [
                "--input-ipc-server=/tmp/mpvsocket", 
                "--ytdl-format=bestvideo[height<=?720]+bestaudio/best",
                "--no-terminal",
                &url,
            ];
            let res = Command::new("mpv")
                .args(&args)
                .spawn();

            match res {
                Ok(_) => log::info!("Succesfully launched player. Playing {}", url),
                Err(e) => log::error!("Error spawning mpv: {}", e),
            }

            let sleep = Duration::from_millis(1000);
            loop {
                match UnixStream::connect("/tmp/mpvsocket") {
                    Ok(stream) => {
                        log::info!("Connected to /tmp/mpvsocket");
                        self.video = Some(stream);
                        break;
                    }
                    Err(_) => {
                        log::error!("Failed to connect to /tmp/mpvsocket. Retrying in 1 second");
                        thread::sleep(sleep);
                    }
                }
            }
        }
        else {
            if let ActivePlayer::VideoPlayer = self.active {
                self.pause_video();
            }
            self.active = ActivePlayer::AudioPlayer;
            match self.audio.command(&["loadfile", url.as_str()]) {
                Ok(_) => log::info!("Succesfully launched player. Playing {}", url),
                Err(e) => log::error!("Error loading {}: {}", url, e),
            }
        }
    }

    pub fn stop(&mut self) {
        match self.active {
            ActivePlayer::VideoPlayer => {
                if let Some(ref mut stream) = self.video {
                    let cmd = b"{ \"command\": [\"stop\"] }\n";
                    match stream.write_all(cmd) {
                        Ok(_) => log::info!("Sent 'stop' command"),
                        Err(e) => log::error!("Failed to send command: {:#?}", e),
                    }
                    self.video = None;
                }
            },
            ActivePlayer::AudioPlayer => {
                match self.audio.command(&["stop"]) {
                    Ok(_) => log::info!("Successfully stopped audio"),
                    Err(e) => log::error!("Unable to stop audio: {}", e),
                }
            },
            _ => {},
        }
    }

    pub fn stop_all(&mut self) {
        self.active = ActivePlayer::AudioPlayer;
        self.stop();
        self.active = ActivePlayer::VideoPlayer;
        self.stop()
    }

    pub fn pause_video(&mut self) {
        if let Some(ref mut stream) = self.video {
            let cmd = b"{ \"command\": [\"set_property\", \"pause\", true] }\n";
            match stream.write_all(cmd) {
                Ok(_) => log::info!("Sent pause command"),
                Err(e) => log::error!("Unable to send command: {:#?}", e),
            }
        }
    }

    pub fn pause_audio(&mut self) {
        match self.audio.set_property("pause", true) {
            Ok(_) => log::info!("Paused audio"),
            Err(e) => log::error!("Unable to pause audio: {}", e)
        }
    }

    pub fn resume_video(&mut self) {
        if let Some(ref mut stream) = self.video {
            let cmd = b"{ \"command\": [\"set_property\", \"pause\", false] }\n";
            stream.write_all(cmd).unwrap();
        }
    }

    pub fn resume_audio(&mut self) {
        match self.audio.set_property("pause", false) {
            Ok(_) => log::info!("Resumed audio"),
            Err(e) => log::error!("Unable to resume audio: {}", e),
        }
    }
    
    pub fn toggle_playback(&mut self) {
        match self.active {
            ActivePlayer::VideoPlayer => {
                if let Some(ref mut stream) = self.video {
                    let cmd = b"{ \"command\": [\"get_property\", \"pause\"] }\n";
                    stream.write_all(cmd).unwrap();

                    let mut buf = [0; 1024];
                    stream.read_exact(&mut buf).unwrap();
                    let json_str = std::str::from_utf8(&buf).unwrap().trim_end_matches('\0');
                    let mpv_response: MpvResponse<bool> = serde_json::from_str(&json_str).unwrap();
                    if mpv_response.data {
                        self.resume_video();
                    }
                    else {
                        self.pause_video();
                    }
                }
            },
            ActivePlayer::AudioPlayer => {
                match self.audio.get_property("pause") {
                    Ok(pause) => {
                        if pause {
                            self.resume_audio();
                        }
                        else {
                            self.pause_audio();
                        }
                    },
                    Err(e) => log::error!("Unable to get pause state: {}", e),
                }
            },
            _ => {},
        }
    }

    pub fn queue(&mut self, url: String) {
        match self.active {
            ActivePlayer::VideoPlayer => {
                if let Some(ref mut stream) = self.video {
                    let cmd = b"{ \"command\": [\"loadfile\", \"append-play\"] }\n";
                    stream.write_all(cmd).unwrap();
                }
            },
            ActivePlayer::AudioPlayer => {
                match self.audio.command(&["loadfile", url.as_str(), "append-play"]) {
                    Ok(_) => log::info!("Succesfully queued track {}", url),
                    Err(e) => log::error!("Unable to queue track {}: {}", url, e),
                }
            }
            _ => {},
        }
    }
    
    pub fn init_audio() -> Result<MpvHandler, Error> {
        let mut mpv_builder = MpvHandlerBuilder::new()?;
        mpv_builder.set_option("osc", true)?;
        mpv_builder.set_option("sid", "no")?;
        mpv_builder.set_option("ytdl-format", "bestvideo[height<=?720]+bestaudio/best")?;
        mpv_builder.set_option("video", "no")?;

        let mut mpv = mpv_builder.build()?;
        mpv.set_property("speed", 1.0)?;
        Ok(mpv)
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            audio: Player::init_audio().unwrap(),
            video: None,
            active: ActivePlayer::NoPlayer,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn toggle_play() {
        use crate::player;
        player::toggle_play_video();
    }
}
