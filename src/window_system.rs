pub type Window = u64;

#[deriving(Show, Clone)]
pub struct Rectangle(pub uint, pub uint, pub uint, pub uint);

impl Rectangle {
    pub fn is_inside(&self, x: uint, y: uint) -> bool {
        let &Rectangle(rx, ry, rw, rh) = self;

        x >= rx && x <= rx + rw && y >= ry && y <= ry + rh 
    }
}

pub struct WindowChanges {
    pub x: uint,
    pub y: uint,
    pub width: uint,
    pub height: uint,
    pub border_width: uint,
    pub sibling: Window,
    pub stack_mode: uint,
}

pub enum WindowSystemEvent {
    ConfigurationNotification(Window),
    ConfigurationRequest(Window, WindowChanges, u64),
    /// A window has been created and needs to be managed.
    WindowCreated(Window),
    /// A window has been destroyed and needs to be unmanaged.
    WindowDestroyed(Window),
    WindowUnmapped(Window, bool),
    /// The pointer has entered a window's area. Mostly used
    /// for mousefollow focus.
    Enter(Window),
    /// The pointer has left a window's area. Mostly used
    /// for mousefollow focus.
    Leave(Window),
    ButtonPressed(Window, uint, uint, uint, uint),
    KeyPressed(Window, uint, uint),
    ClientMessageEvent(Window),
    /// The underlying event by xlib or wayland is unknown
    /// and can be ignored.
    UnknownEvent
}

pub trait WindowSystem {
    fn get_root(&self) -> Window;
    /// Retrieve geometry infos over all screens
    fn get_screen_infos(&self) -> Vec<Rectangle>;
    /// Get the number of physical displays
    fn get_number_of_screens(&self) -> uint;
    /// Get the width of the given physical screen 
    fn get_display_width(&self, screen: uint) -> u32;
    /// Get the height of the given physical screen
    fn get_display_height(&self, screen: uint) -> u32;
    /// Get the given window's name
    fn get_window_name(&self, window: Window) -> String;
    /// Get a list of all windows
    fn get_windows(&self) -> Vec<Window>;
    /// Set the given window's border width
    fn set_window_border_width(&mut self, window: Window, border_width: uint);
    /// Set the given window's border color
    fn set_window_border_color(&mut self, window: Window, border_color: uint);
    /// Resize the window to the given dimensions
    fn resize_window(&mut self, window: Window, width: uint, height: uint);
    /// Move the window's top left corner to the given coordinates
    fn move_window(&mut self, window: Window, x: uint, height: uint);
    /// Map the window to the screen and show it
    fn show_window(&mut self, window: Window);
    fn hide_window(&mut self, window: Window);
    fn focus_window(&mut self, window: Window);
    fn configure_window(&mut self, window: Window, window_changes: WindowChanges, mask: u64);
    /// Check if there are events pending
    fn event_pending(&self) -> bool;
    /// Get the next event from the queue
    fn get_event(&mut self) -> WindowSystemEvent;
    fn flush(&mut self);
}


