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

#[derive(Deserialize, Debug)]
struct MpvEvent {
    event: String
}

pub struct Player {
    audio: MpvHandler,
    video: Option<UnixStream>,
}

impl Player {
    pub fn audio_changed(&mut self) -> bool {
        if let Some(event) = self.audio.wait_event(0.0) {
            if let Event::StartFile = event {
                return true
            }
        }
        false
    }

    pub fn video_changed(&mut self) -> bool {
        if let Some(ref mut stream) = self.video {
            let mut buf = [0; 1024];
            if stream.read(&mut buf).is_ok() {
                let json_str = std::str::from_utf8(&buf).unwrap().trim_end_matches('\0');
                log::debug!("Event = {}", json_str);
                if json_str.contains("end-file"){
                    return false
                }
                
                if json_str.contains("tracks-changed") {
                    return true
                }
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

    pub fn seek_audio(&mut self, seek_amnt: &str) {
        match self.audio.command(&["seek", seek_amnt]) {
            Ok(_) => log::info!("Successfully moved {} seconds", seek_amnt),
            Err(e) => log::error!("Unable to seek audio: {}", e),
        }
    }
    
    pub fn play(&mut self, url: String, is_video: bool) {
        if is_video {
            self.pause_audio();
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
                        stream.set_nonblocking(true).unwrap();
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
            self.pause_video();
            
            match self.audio.command(&["loadfile", url.as_str()]) {
                Ok(_) => log::info!("Succesfully launched player. Playing {}", url),
                Err(e) => log::error!("Error loading {}: {}", url, e),
            }
            self.resume_audio();
        }
    }
    
    pub fn toggle_loop_audio(&mut self) {
        match self.audio.get_property::<&str>("loop") {
            Ok(loop_value) => {
                if loop_value == "no" {
                    self.audio.set_option("loop", "inf").unwrap();
                }
                else {
                    self.audio.set_option("loop", "no").unwrap();
                }
            },
            Err(e) => log::error!("Unable to toggle loop: {}", e),
        }
    }

    pub fn stop_video(&mut self) {
        let cmd = "{ \"command\": [\"stop\"] }\n";
        self.send_video_command(cmd);
        self.video = None;
    }

    pub fn stop_audio(&mut self) {
        match self.audio.command(&["stop"]) {
            Ok(_) => log::info!("Successfully stopped audio"),
            Err(e) => log::error!("Unable to stop audio: {}", e),
        }
    }

    pub fn stop_all(&mut self) {
        self.stop_audio();
        self.stop_video()
    }

    pub fn pause_video(&mut self) {
        let cmd = "{ \"command\": [\"set_property\", \"pause\", true] }\n";
        self.send_video_command(cmd);
    }

    pub fn pause_audio(&mut self) {
        match self.audio.set_property("pause", true) {
            Ok(_) => log::info!("Paused audio"),
            Err(e) => log::error!("Unable to pause audio: {}", e)
        }
    }

    pub fn _resume_video(&mut self) {
        if let Some(ref mut stream) = self.video {
            let cmd = b"{ \"command\": [\"set_property\", \"pause\", false] }\n";
            match stream.write_all(cmd) {
                Ok(_) => log::info!("Sent pause command"),
                Err(e) => {
                    log::error!("Unable to send command: {:#?}", e);
                    self.video = None;
                },
            }
        }
    }

    pub fn resume_audio(&mut self) {
        match self.audio.set_property("pause", false) {
            Ok(_) => log::info!("Resumed audio"),
            Err(e) => log::error!("Unable to resume audio: {}", e),
        }
    }

    pub fn toggle_audio_playback(&mut self) {
        match self.audio.get_property("pause") {
            Ok(pause) => {
                if pause {
                    if self.video.is_some() {
                        self.pause_video();
                    }
                    
                    self.resume_audio();
                }
                else {
                    self.pause_audio();
                }
            },
            Err(e) => log::error!("Unable to get pause state: {}", e),
        }
    }

    pub fn _toggle_video_playback(&mut self) {
        if let Some(ref mut stream) = self.video {
            let cmd = b"{ \"command\": [\"get_property\", \"pause\"] }\n";
            stream.write_all(cmd).unwrap();
            let mut buf = [0; 1024];
            stream.read_exact(&mut buf).unwrap();
            let json_str = std::str::from_utf8(&buf).unwrap().trim_end_matches('\0');
            let mpv_response: MpvResponse<bool> = serde_json::from_str(&json_str).unwrap();
            if mpv_response.data {
                self._resume_video();
            }
            else {
                self.pause_video();
            }
        }
    }

    pub fn queue_audio(&mut self, url: String) {
        match self.audio.command(&["loadfile", url.as_str(), "append-play"]) {
            Ok(_) => log::info!("Succesfully queued track {}", url),
            Err(e) => log::error!("Unable to queue track {}: {}", url, e),
        }
    }
    
    pub fn queue_video(&mut self, url: String) {
        let cmd = format!("{{ \"command\": [\"loadfile\", \"{}\", \"append-play\"] }}\n", url);
        self.send_video_command(&cmd);
    }

    pub fn get_status(&self) -> String {
        match self.audio.get_property::<&str>("idle-active") {
            Ok(is_idle) => {
                if is_idle == "yes"{
                    return String::from("Idle")
                }
                match self.audio.get_property::<bool>("pause") {
                    Ok(is_paused) => { 
                        if is_paused {
                            return String::from("Paused") 
                        }
                        
                        let is_looped = match self.audio.get_property::<&str>("loop") {
                            Ok(loop_value) => {
                                if loop_value != "no" {
                                    "(Looped)"
                                }
                                else {
                                    ""
                                }
                            },
                            Err(e) => {
                                log::error!("Unable to toggle loop: {}", e);
                                ""
                            },
                        };
                        format!("Playing {}", is_looped)
                    },
                    Err(_) => String::from("Idle"),
                }
            },
            Err(_) => String::from("Idle")
        }
    }

    pub fn get_time(&self) -> String {
        let time = match self.audio.get_property::<i64>("playback-time") {
            Ok(time) => {
                let seconds = time;
                let hours = seconds / 3600;
                let seconds = seconds % 3600;
                let minutes = seconds / 60;
                let seconds = seconds % 60;
                format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
            },
            Err(_) => String::from("00:00:00")
        };

        let duration = match self.audio.get_property::<i64>("duration") {
            Ok(time) => {
                let seconds = time;
                let hours = seconds / 3600;
                let seconds = seconds % 3600;
                let minutes = seconds / 60;
                let seconds = seconds % 60;
                format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
            },
            Err(_) => String::from("00:00:00"),
        };

        format!("{} / {}", time, duration)
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

    fn send_video_command(&mut self, cmd: &str) {
        if let Some(ref mut stream) = self.video {
            match stream.write_all(cmd.as_bytes()) {
                Ok(_) => log::info!("Sent command: {:#?}", cmd),
                Err(e) => log::error!("Unable to send command: {:#?}", e),
            }
        }
    }

}

impl Default for Player {
    fn default() -> Self {
        Player {
            audio: Player::init_audio().unwrap(),
            video: None,
        }
    }
}
