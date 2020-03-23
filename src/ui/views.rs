use tui::layout::Constraint;
use std::sync::{
    Arc,
    RwLock,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ViewType {
    Search,
    Home,
    Queue,
}

#[derive(Clone, Debug)]
pub enum ContentType {
    MediaContent(Arc<RwLock<Vec<Vec<String>>>>),
    ListContent(Vec<String>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum WindowType {
    PlaylistVideos,
    TrendingVideos,
    TopVideos,
    PopularVideos,
    SearchVideos,
    SearchChannels,
    SearchPlaylists,
    ChannelVideos,
    ChannelPlaylists,
    VideoQueue,
    AudioQueue,
}

#[derive(Clone, Debug)]
pub struct TabState {
    pub selected: usize,
    pub items: Vec<String>,
    pub title: String,
}

impl TabState {
    pub fn new(items: Vec<String>, title: String) -> Self {
        TabState {
            title,
            items,
            selected: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct View {
    pub tabs: TabState,
    pub view_stack: Vec<View>,
    pub root_windows: Vec<Window>,
}

impl View {
    pub fn new(root_windows: Vec<Window>, tabs: Vec<String>, tab_title: String) -> Self {
        View {
            tabs: TabState::new(tabs, tab_title),
            view_stack: vec![],
            root_windows,
        }
    }

    pub fn get_current_view(&self) -> Option<&View> {
        if !self.view_stack.is_empty() {
            self.view_stack.last()
        }
        else {
            //self.root_windows.get(self.tabs.selected)
            Some(self)
        }
    }
    
    pub fn get_current_view_mut(&mut self) -> Option<&mut View> {
        if !self.view_stack.is_empty() {
            self.view_stack.last_mut()
        }
        else {
            //self.root_windows.get_mut(self.tabs.selected)
            Some(self)
        }
    }

    pub fn pop_stack(&mut self) {
        if !self.view_stack.is_empty() {
            self.view_stack.pop(); 
        }
    }
}

#[derive(Clone, Debug)]
pub struct Window {
    pub title: String,
    pub selected: usize,
    pub content: ContentType,
    pub headers: Option<Box<[&'static str]>>,
    pub window_type: WindowType,
    pub column_widths: Box<[Constraint]>
}

impl Window {
    pub fn new(title: String, 
               selected: usize,
               content: ContentType,
               headers: Option<Box<[&'static str]>>,
               window_type: WindowType,
               column_widths: Box<[Constraint]>,
               ) -> Self 
    {
        Window {
            title,
            selected,
            content,
            headers,
            window_type,
            column_widths
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        Window {
            title: String::new(),
            selected: 0,
            content: ContentType::ListContent(vec![]),
            headers: None,
            window_type: WindowType::SearchVideos,
            column_widths: Box::new([]),
        }
    }
}

