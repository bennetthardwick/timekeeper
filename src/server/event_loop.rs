use crate::Server;

pub struct EventLoop<'a> {
    server: &'a Server,
}

#[derive(Debug)]
pub enum Event {
    FocusChange,
}

impl<'a> Iterator for EventLoop<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let event = self.server.connection.wait_for_event()?;

            let e = match event.response_type() {
                xcb::PROPERTY_NOTIFY => self.on_property_notify(unsafe { xcb::cast_event(&event) }),
                num => {
                    println!("Unknown event! {}", num);
                    None
                }
            };

            if let Some(event) = e {
                return Some(event);
            }
        }
    }
}

impl<'a> EventLoop<'a> {
    pub(crate) fn new(server: &Server) -> EventLoop {
        EventLoop { server }
    }

    fn on_property_notify(&self, event: &xcb::PropertyNotifyEvent) -> Option<Event> {
        Some(Event::FocusChange)
    }
}
