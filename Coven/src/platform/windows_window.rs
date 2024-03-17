use std::time::Duration;

use crate::{
    event::{
        application_event::{WindowCloseEvent, WindowFocusEvent, WindowLostFocusEvent, WindowMovedEvent, WindowResizeEvent}, key_event::{KeyPressedEvent, KeyReleasedEvent}, mouse_event::{MouseButtonPressedEvent, MouseButtonReleasedEvent, MouseMovedEvent, MouseScrolledEvent}, IEventListener
    },
    window::{Window, WindowProps},
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, Event as WinitEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::pump_events::EventLoopExtPumpEvents,
    window::{Window as WinitWindow, WindowBuilder},
};

pub struct WindowsWindow<'window_life, T: IEventListener> {
    props: WindowProps,
    window: WinitWindow,
    event_loop: EventLoop<()>,
    event_cb: Option<&'window_life T>,
}

impl<'window_life, T: IEventListener> Window<'window_life, T> for WindowsWindow<'window_life, T> {
    fn on_update(&mut self) {
        self.event_loop
            .pump_events(Some(Duration::ZERO), |event, elwt| {
                match event {
                    WinitEvent::AboutToWait => {
                        // Render Here
                    },
                    // --------------- Window Events ---------------
                    WinitEvent::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        elwt.exit();
                        let mut close_event = WindowCloseEvent::new();
                        //TODO: Safe unwrap when no callback is set
                        self.event_cb.unwrap().on_event(&mut close_event);
                        //    std::process::exit(0);
                    },
                    WinitEvent::WindowEvent {
                        event: WindowEvent::Resized(size),
                        ..
                    } => {
                        self.props.height = size.height;
                        self.props.width = size.width;
                        let mut resize_event = WindowResizeEvent::new(size.height, size.width);
                        self.event_cb.unwrap().on_event(&mut resize_event);
                    },
                    WinitEvent::WindowEvent {
                        event: WindowEvent::Moved(pos),
                        ..
                    } => {
                        let mut window_moved_event = WindowMovedEvent::new(pos.x as f32, pos.y as f32);
                        self.event_cb.unwrap().on_event(&mut window_moved_event);
                    },
                    WinitEvent::WindowEvent {
                        event: WindowEvent::Focused(is_focused),
                        ..
                    } => {
                        if is_focused {
                            let mut window_focus_event = WindowFocusEvent::new();
                            self.event_cb.unwrap().on_event(&mut window_focus_event);
                        } else {
                            let mut window_focus_event = WindowLostFocusEvent::new();
                            self.event_cb.unwrap().on_event(&mut window_focus_event);
                        }
                        
                    },
                    // --------------- Mouse Events ---------------
                    WinitEvent::WindowEvent {
                        event: WindowEvent::CursorMoved {
                            position: pos,
                            ..
                        },
                        ..
                    } => {
                        let mut mouse_moved_event = MouseMovedEvent::new(pos.x as f32, pos.y as f32);
                        self.event_cb.unwrap().on_event(&mut mouse_moved_event);
                    },
                    WinitEvent::WindowEvent {
                        event: WindowEvent::MouseInput {
                            state,
                            button,
                            ..
                        },
                        ..
                    } => {
                        match state {
                            ElementState::Pressed => {
                                let mut mouse_btn_event;
                                //TODO: Maybe use enum for these
                                match button {
                                    MouseButton::Left => {
                                        mouse_btn_event = MouseButtonPressedEvent::new(1);
                                    },
                                    MouseButton::Right => {
                                        mouse_btn_event = MouseButtonPressedEvent::new(2);
                                    },
                                    MouseButton::Middle => {
                                        mouse_btn_event = MouseButtonPressedEvent::new(3);
                                    },
                                    MouseButton::Back => {
                                        mouse_btn_event = MouseButtonPressedEvent::new(4);
                                    },
                                    MouseButton::Forward => {
                                        mouse_btn_event = MouseButtonPressedEvent::new(5);
                                    },
                                    MouseButton::Other(num) => {
                                        mouse_btn_event = MouseButtonPressedEvent::new(num as i32);
                                    },
                                }

                                
                                self.event_cb.unwrap().on_event(&mut mouse_btn_event);
                            },
                            ElementState::Released => {
                                let mut mouse_btn_event;
                                //TODO: Maybe use enum for these
                                match button {
                                    MouseButton::Left => {
                                        mouse_btn_event = MouseButtonReleasedEvent::new(1);
                                    },
                                    MouseButton::Right => {
                                        mouse_btn_event = MouseButtonReleasedEvent::new(2);
                                    },
                                    MouseButton::Middle => {
                                        mouse_btn_event = MouseButtonReleasedEvent::new(3);
                                    },
                                    MouseButton::Back => {
                                        mouse_btn_event = MouseButtonReleasedEvent::new(4);
                                    },
                                    MouseButton::Forward => {
                                        mouse_btn_event = MouseButtonReleasedEvent::new(5);
                                    },
                                    MouseButton::Other(num) => {
                                        mouse_btn_event = MouseButtonReleasedEvent::new(num as i32);
                                    },
                                }

                                
                                self.event_cb.unwrap().on_event(&mut mouse_btn_event);
                            }
                        }
                        

                    },
                    WinitEvent::WindowEvent {
                        event: WindowEvent::MouseWheel {
                            delta,
                            ..
                        },
                        ..
                    } => {
                        match delta {
                            //TODO: Add information to event so that we can tell if it's a scroll lines delta or pixel offset
                            MouseScrollDelta::LineDelta(x, y) => {
                                let mut mouse_moved_event = MouseScrolledEvent::new(x, y);
                                self.event_cb.unwrap().on_event(&mut mouse_moved_event);
                            },
                            MouseScrollDelta::PixelDelta(pos) => {
                                let mut mouse_moved_event = MouseScrolledEvent::new(pos.x as f32, pos.y as f32);
                                self.event_cb.unwrap().on_event(&mut mouse_moved_event);
                            }
                        }
                       
                    },
                    // --------------- Keyboard Events ---------------
                    WinitEvent::WindowEvent {
                        event: WindowEvent::KeyboardInput {
                            event,
                            ..
                        },
                        ..
                    } => {
                        match event.state {
                            ElementState::Pressed => {
                                match event.physical_key {
                                    winit::keyboard::PhysicalKey::Code(key_code) => {
                                        if !event.repeat {
                                            //TODO: make custom handler to proccess key code
                                            let mut key_pressed_event = KeyPressedEvent::new(crate::event::key_event::KeyCode::key_code_from_winint(key_code), 0);
                                            self.event_cb.unwrap().on_event(&mut key_pressed_event);
                                        } else {
                                            let mut key_pressed_event = KeyPressedEvent::new(crate::event::key_event::KeyCode::key_code_from_winint(key_code), 1);
                                            self.event_cb.unwrap().on_event(&mut key_pressed_event);
                                        }
                                    }
                                    winit::keyboard::PhysicalKey::Unidentified(key_code) => {
                                        match key_code {
                                            winit::keyboard::NativeKeyCode::Unidentified => {},
                                            winit::keyboard::NativeKeyCode::Android(_android_code) => {
                                                // TODO: support this at some point
                                            },
                                            winit::keyboard::NativeKeyCode::MacOS(_mac_code) => {
                                                // TODO: support this at some point
                                            },
                                            winit::keyboard::NativeKeyCode::Windows(_scan_code) => {
                                                // TODO: support this at some point
                                            },
                                            winit::keyboard::NativeKeyCode::Xkb(_) => {
                                                // TODO: Add support for Xkb in future
                                            },
                                        }
                                    },
                                }
                            },
                            ElementState::Released => {
                                match event.physical_key {
                                    winit::keyboard::PhysicalKey::Code(key_code) => {
                                        let mut key_released_event = KeyReleasedEvent::new(crate::event::key_event::KeyCode::key_code_from_winint(key_code));
                                        self.event_cb.unwrap().on_event(&mut key_released_event); 
                                    },
                                    winit::keyboard::PhysicalKey::Unidentified(_) => {},
                                }

                            }
                        }
                       
                    },
                    //TODO: Parse KeyModifiers
                    _ => (),
                }
            });
    }

    fn get_height(&self) {
        self.props.height;
    }

    fn get_width(&self) {
        self.props.width;
    }

    fn set_event_callback(&mut self, event_cb: &'window_life T) {
        self.event_cb = Option::from(event_cb);
    }

    fn set_vsync(&mut self, _enabled: bool) {
        todo!()
    }

    fn is_vsync(&self) -> bool {
        todo!()
    }

    fn create(props: &WindowProps) -> Self
    where
        Self: Sized,
    {
        //TODO: Remove unwraps. Look into not cloning data where possible
        let event_loop = EventLoop::new().unwrap();
        let primary_monitor = event_loop.primary_monitor().unwrap();
        let width = primary_monitor.size().width;
        let height = primary_monitor.size().height;
        event_loop.set_control_flow(ControlFlow::Poll);
        let mut window_height_pos = height - props.height;
        if window_height_pos > 0 {
            window_height_pos -= 32
        }
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(props.width, props.height))
            .with_title(props.title.clone())
            .with_position(PhysicalPosition::new(
                (width - props.width) / 2,
                (window_height_pos) / 2,
            ))
            .build(&event_loop)
            .unwrap();
        let window_props = props.clone();
        WindowsWindow {
            props: window_props,
            window,
            event_loop,
            event_cb: None,
        }
    }
}
