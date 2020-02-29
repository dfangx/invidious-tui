use std::sync::{
    Arc,
    RwLock,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ViewType {
    Search,
    Home,
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
}

#[derive(Clone, Debug)]
pub struct TabState {
    pub selected: usize,
    pub items: Vec<String>,
}

impl TabState {
    pub fn new(items: Vec<String>) -> Self {
        TabState {
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
    pub fn new(root_windows: Vec<Window>, tabs: Vec<String>) -> Self {
        View {
            tabs: TabState::new(tabs),
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
    pub headers: Option<Vec<String>>,
    pub window_type: WindowType,
}

impl Window {
    pub fn new(title: String, 
               selected: usize,
               content: ContentType,
               headers: Option<Vec<String>>,
               window_type: WindowType,
               ) -> Self 
    {
        Window {
            title,
            selected,
            content,
            headers,
            window_type,
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
        }
    }
}

