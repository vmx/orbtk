use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};

use libremarkable::framebuffer::common::{
    color, display_temp, dither_mode, mxcfb_rect, waveform_mode, DISPLAYHEIGHT, DISPLAYWIDTH,
};
use libremarkable::framebuffer::core::Framebuffer;
use libremarkable::framebuffer::refresh::PartialRefreshMode;
use libremarkable::framebuffer::{FramebufferIO, FramebufferRefresh};
use libremarkable::input::multitouch::MultitouchEvent;
use libremarkable::input::InputEvent;

use crate::{
    event::{ButtonState, MouseButton, MouseEvent},
    render::RenderContext2D,
    window_adapter::WindowAdapter,
    WindowRequest,
};

use orbtk_utils::{Point, Rectangle};

/// Represents a wrapper for a orbclient window. It handles events, propagate them to
/// the window adapter and handles the update and render pipeline.
pub struct Window<A>
where
    A: WindowAdapter,
{
    framebuffer: Framebuffer<'static>,
    input_receiver: mpsc::Receiver<InputEvent>,
    adapter: A,
    render_context: RenderContext2D,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>,
    update: bool,
    redraw: Arc<AtomicBool>,
    close: bool,
    dirty_region: Option<Rectangle>,
}

impl<A> Window<A>
where
    A: WindowAdapter,
{
    pub fn new(
        framebuffer: Framebuffer<'static>,
        input_receiver: mpsc::Receiver<InputEvent>,
        adapter: A,
        render_context: RenderContext2D,
        request_receiver: Option<mpsc::Receiver<WindowRequest>>,
    ) -> Self {
        Window {
            framebuffer,
            input_receiver,
            adapter,
            render_context,
            request_receiver,
            update: true,
            redraw: Arc::new(AtomicBool::new(true)),
            close: false,
            dirty_region: None,
        }
    }
}

impl<A> Window<A>
where
    A: WindowAdapter,
{
    /// Check if the window is open.
    pub fn is_open(&self) -> bool {
        !self.close
    }

    /// Updates the clipboard.
    pub fn update_clipboard(&mut self) {}

    /// Drain events and propagate the events to the adapter.
    pub fn drain_events(&mut self) {
        for input_event in self.input_receiver.try_iter() {
            println!("input event: {:?}", input_event);
            match input_event {
                InputEvent::MultitouchEvent {
                    event: MultitouchEvent::Press { finger },
                } => {
                    let mouse_event = MouseEvent {
                        position: Point::new(finger.pos.x as f64, finger.pos.y as f64),
                        button: MouseButton::Left,
                        state: ButtonState::Down,
                    };
                    println!("mouse event press: {:?}", mouse_event);
                    self.adapter.mouse_event(mouse_event);
                }
                InputEvent::MultitouchEvent {
                    event: MultitouchEvent::Release { finger },
                } => {
                    let mouse_event = MouseEvent {
                        position: Point::new(finger.pos.x as f64, finger.pos.y as f64),
                        button: MouseButton::Left,
                        state: ButtonState::Up,
                    };
                    println!("mouse event release: {:?}", mouse_event);
                    self.adapter.mouse_event(mouse_event);
                }
                _ => (),
            }
        }
    }

    /// Receives window request from the application and handles them.
    pub fn receive_requests(&mut self) {
        if let Some(request_receiver) = &self.request_receiver {
            for request in request_receiver.try_iter() {
                match request {
                    WindowRequest::Redraw => {
                        if !self.update && !self.redraw.load(Ordering::Relaxed) {
                            self.update = true;
                            self.redraw.store(true, Ordering::Relaxed)
                        }
                    }
                    WindowRequest::ChangeTitle(_) => {
                        // On the reMarkable there is no window title
                    }
                    WindowRequest::Close => {
                        self.close = true;
                    }
                }
            }
        }
    }

    /// Runs update on the adapter.
    pub fn update(&mut self) {
        //super::CONSOLE.time("complete");
        if !self.update {
            return;
        }

        self.dirty_region = self.adapter.run(&mut self.render_context);
        self.update = false;
        self.redraw.store(true, Ordering::Relaxed)
    }

    /// Swaps the current frame buffer.
    pub fn render(&mut self) {
        if self.redraw.load(Ordering::Relaxed) {
            let bytes = self.render_context.data_u8_mut();

            if let Some(dirty_region) = self.dirty_region {
                println!(
                    "vmx: orbclient: window: render: dirty region: {:?}",
                    dirty_region
                );

                // ARGB to RGB565
                let remarkable_buffer: Vec<u8> = bytes
                    .chunks(4)
                    .flat_map(|pixel| {
                        let (red, green, blue) = (pixel[1], pixel[2], pixel[3]);
                        let col = color::RGB(red, green, blue);
                        col.as_native().to_vec()
                    })
                    .collect();

                // The framebuffer is bigger than the canvas that can display something (it's 4 bytes
                // longer per line), hence don't put it into the framebuffer directly, but use the
                // `restore_region()` call.
                self.framebuffer
                    .restore_region(
                        mxcfb_rect {
                            top: 0,
                            left: 0,
                            width: DISPLAYWIDTH.into(),
                            height: DISPLAYHEIGHT.into(),
                        },
                        &remarkable_buffer,
                    )
                    .unwrap();

                self.redraw.store(false, Ordering::Relaxed);

                //self.framebuffer.full_refresh(
                //    waveform_mode::WAVEFORM_MODE_GC16_FAST,
                //    display_temp::TEMP_USE_AMBIENT,
                //    dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                //    0,
                //    true,
                //);
                let rect = mxcfb_rect {
                    left: dirty_region.x() as u32,
                    top: dirty_region.y() as u32,
                    width: dirty_region.width() as u32,
                    height: dirty_region.height() as u32,
                };
                self.framebuffer.partial_refresh(
                    &rect,
                    //PartialRefreshMode::Wait,
                    PartialRefreshMode::Async,
                    waveform_mode::WAVEFORM_MODE_GC16_FAST,
                    //display_temp::TEMP_USE_MAX,
                    display_temp::TEMP_USE_REMARKABLE_DRAW,
                    dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                    0,
                    false,
                );
            }
        }
    }
}
