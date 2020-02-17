mod event_loop;

pub use event_loop::*;
use std::convert::TryInto;
use xcb_util::ewmh::{ get_wm_name, get_active_window };

pub struct WindowSnapshot {}

pub struct Server {
    connection: xcb_util::ewmh::Connection,
    root: i32,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Server {
    pub fn new() -> Result<Server> {
        let (connection, _screen) = xcb::Connection::connect(None)?;

        let connection = xcb_util::ewmh::Connection::connect(connection)
            .map_err(|(_a, _b)| "Could not create ewmh connection")?;

        let screen = connection
            .get_setup()
            .roots()
            .nth(0)
            .ok_or("Failed to get screen")?;

        let root: i32 = screen.root().try_into()?;

        println!("Root window: {:?}", root);

        Ok(Server { connection, root })
    }

    pub fn enable_window_events(&self) -> Result<()> {
        let mask = [
            (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_PROPERTY_CHANGE),
        ];
        xcb::change_window_attributes_checked(&self.connection, self.root as u32, &mask)
            .request_check()?;
        Ok(())
    }

    pub fn events<'a>(&'a self) -> EventLoop<'a> {
        EventLoop::new(&self)
    }

    pub fn snapshot_active_window(&self, window: xcb::Window) -> Result<WindowSnapshot> {
        let active = get_active_window(&self.connection, window.try_into().unwrap());
        let window = active.get_reply()?;

        // let name = get_wm_name(&self.connection, window).get_reply()?;

        // println!("Name: {}", name.string());

        Ok(WindowSnapshot {})
    }

    pub fn flush(&self) {}
}
