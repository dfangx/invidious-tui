use tui::layout::Constraint;

pub const VIDEO_HEADERS: [&str; 4] = ["Title", "Author", "Uploaded", "Duration"];
pub const PLAYLIST_HEADERS: [&str; 3] = ["Name", "Author", "# of Videos"];
pub const CHANNEL_HEADERS: [&str; 3] = ["Name", "# of Subs", "# of Videos"];
pub const PLAYLIST_VIDEO_HEADERS: [&str; 3] = ["Title", "Author", "Duration"];
pub const QUEUE_HEADERS: [&str; 2] = ["Title", "Author"];
pub const VIDEO_COLUMN_CONSTRAINTS: [Constraint; 4] = [
    Constraint::Percentage(60), 
    Constraint::Length(30),
    Constraint::Length(20),
    Constraint::Length(10),
];
pub const QUEUE_COLUMN_CONSTRAINTS: [Constraint; 2] = [
    Constraint::Percentage(60),
    Constraint::Percentage(40),
];
pub const DEFAULT_COLUMN_CONSTRAINTS: [Constraint; 3] = [
    Constraint::Percentage(60), 
    Constraint::Length(30),
    Constraint::Length(10),
];
