use std::{collections::HashMap, sync::mpsc};

use libremarkable::framebuffer::core::Framebuffer;
use libremarkable::framebuffer::FramebufferBase;
use libremarkable::input::InputDevice;
use libremarkable::input::{ev::EvDevContext, InputEvent};

use super::{Shell, Window};
use crate::{
    render::RenderContext2D, utils::Rectangle, window_adapter::WindowAdapter, WindowRequest,
    WindowSettings,
};

/// The `WindowBuilder` is used to construct a window shell for the minifb backend.
pub struct WindowBuilder<'a, A: 'static>
where
    A: WindowAdapter,
{
    shell: &'a mut Shell<A>,
    adapter: A,
    fonts: HashMap<String, &'static [u8]>,
    bounds: Rectangle,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>,
}

impl<'a, A> WindowBuilder<'a, A>
where
    A: WindowAdapter,
{
    /// Creates a new window builder.
    pub fn new(shell: &'a mut Shell<A>, adapter: A) -> Self {
        WindowBuilder {
            shell,
            adapter,
            fonts: HashMap::new(),
            bounds: Rectangle::new((0.0, 0.0), (100.0, 75.0)),
            request_receiver: None,
        }
    }

    /// Creates the window builder from a settings object.
    pub fn from_settings(settings: WindowSettings, shell: &'a mut Shell<A>, adapter: A) -> Self {
        WindowBuilder {
            shell,
            adapter,
            fonts: settings.fonts,
            bounds: Rectangle::new(settings.position, (settings.size.0, settings.size.1)),
            request_receiver: None,
        }
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rectangle>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Registers a new font with family key.
    pub fn font(mut self, family: impl Into<String>, font_file: &'static [u8]) -> Self {
        self.fonts.insert(family.into(), font_file);
        self
    }

    /// Register a window request receiver to communicate with the window shell from outside.
    pub fn request_receiver(mut self, request_receiver: mpsc::Receiver<WindowRequest>) -> Self {
        self.request_receiver = Some(request_receiver);
        self
    }

    /// Builds the window shell and add it to the application `Shell`.
    pub fn build(self) {
        let mut render_context = RenderContext2D::new(self.bounds.width(), self.bounds.height());

        let framebuffer = Framebuffer::from_path("/dev/fb0");

        let (input_sender, input_receiver) = mpsc::channel::<InputEvent>();
        EvDevContext::new(InputDevice::GPIO, input_sender.clone()).start();
        EvDevContext::new(InputDevice::Multitouch, input_sender.clone()).start();
        EvDevContext::new(InputDevice::Wacom, input_sender).start();

        for (family, font) in self.fonts {
            render_context.register_font(&family, font);
        }

        self.shell.window_shells.push(Window::new(
            framebuffer,
            input_receiver,
            self.adapter,
            render_context,
            self.request_receiver,
        ));
    }
}
