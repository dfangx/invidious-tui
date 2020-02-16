/*
use crate::traits::ListItem;
use crate::traits::ViewExt;
*/

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Channel {
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub video_count: u64,
    pub sub_count: u64,
}

/*
impl ListItem for Channel {
    fn right_text(&self) -> String {
        format!("{} videos", self.video_count)
    }

    fn left_text(&self) -> String {
        format!("[{} subs] {}", self.sub_count, self.author)
    }

    fn play(&self) {
        //let url = self.url.clone();
        //std::thread::spawn(move || {
        //    Command::new("mpv")
        //        .arg(&url)
        //        .output()
        //        .expect("Failed to start playlist");
        //});
    }

    fn open(&self) -> Option<Box<dyn ViewExt>> {
        None
    }

    fn download(&self) {
        //let url = self.url.clone();
        //std::thread::spawn(move || {
        //    Command::new("youtube-dl")
        //        .arg("-o")
        //        .arg("~/downloads/%(playlist_title)s_%(playlist_uploader)s/(title)s_%(channel)s.%(ext)s")
        //        .arg(&url)
        //        .output()
        //        .expect("Failed to download video");
        //});
    }

    fn download_audio_only(&self) {
        //let url = self.url.clone();
        //std::thread::spawn(move || {
        //    Command::new("youtube-dl")
        //        .arg("-o")
        //        .arg("~/downloads/%(playlist_title)s_%(playlist_uploader)s/%(title)s_%(channel)s.%(ext)s")
        //        .arg("-f140")
        //        .arg(&url)
        //        .output()
        //        .expect("Failed to download video");
        //});
    }
}
*/
