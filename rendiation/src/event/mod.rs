use crate::renderer::WGPURenderer;
use winit::event;
use winit::event::*;

type ListenerContainer<AppState> = Vec<Box<dyn FnMut(&mut AppState, &mut WGPURenderer)>>;

pub struct WindowEventSession<AppState> {
  events_cleared_listeners: ListenerContainer<AppState>,
  mouse_down_listeners: ListenerContainer<AppState>,
  mouse_motion_listeners: ListenerContainer<AppState>,
  mouse_wheel_listeners: ListenerContainer<AppState>,
  resize_listeners: ListenerContainer<AppState>,
}

fn emit_listener<AppState>(
  listeners: &mut ListenerContainer<AppState>,
  state: &mut AppState,
  renderer: &mut WGPURenderer,
) {
  for listener in listeners.iter_mut() {
    listener(state, renderer)
  }
}

impl<AppState> WindowEventSession<AppState> {
  pub fn add_mouse_down_listener<T: FnMut(&mut AppState, &mut WGPURenderer) + 'static>(
    &mut self,
    func: T,
  ) {
    self.mouse_down_listeners.push(Box::new(func));
  }

  pub fn add_resize_listener<T: FnMut(&mut AppState, &mut WGPURenderer) + 'static>(
    &mut self,
    func: T,
  ) {
    self.resize_listeners.push(Box::new(func));
  }

  pub fn add_events_clear_listener<T: FnMut(&mut AppState, &mut WGPURenderer) + 'static>(
    &mut self,
    func: T,
  ) {
    self.events_cleared_listeners.push(Box::new(func));
  }

  pub fn add_mouse_wheel_listener<T: FnMut(&mut AppState, &mut WGPURenderer) + 'static>(
    &mut self,
    func: T,
  ) {
    self.mouse_wheel_listeners.push(Box::new(func));
  }

  pub fn add_mouse_motion_listener<T: FnMut(&mut AppState, &mut WGPURenderer) + 'static>(
    &mut self,
    func: T,
  ) {
    self.mouse_motion_listeners.push(Box::new(func));
  }

  pub fn new() -> Self {
    Self {
      events_cleared_listeners: Vec::new(),
      mouse_down_listeners: Vec::new(),
      mouse_motion_listeners: Vec::new(),
      mouse_wheel_listeners: Vec::new(),
      resize_listeners: Vec::new(),
    }
  }

  pub fn event(
    &mut self,
    event: winit::event::Event<()>,
    s: &mut AppState,
    renderer: &mut WGPURenderer,
  ) {
    match event {
      event::Event::WindowEvent { event, .. } => match event {
        WindowEvent::Resized(size) => {
          emit_listener(&mut self.resize_listeners, s, renderer);
          log::info!("Resizing to {:?}", size);
        }
        WindowEvent::MouseInput { button, state, .. } => match button {
          MouseButton::Left => match state {
            ElementState::Pressed => emit_listener(&mut self.mouse_wheel_listeners, s, renderer),
            ElementState::Released => (),
          },
          MouseButton::Right => match state {
            ElementState::Pressed => (),
            ElementState::Released => (),
          },
          _ => {}
        },
        WindowEvent::MouseWheel { delta, .. } => {
          if let MouseScrollDelta::LineDelta(x, y) = delta {
            emit_listener(&mut self.mouse_wheel_listeners, s, renderer);
          }
        }
        WindowEvent::CursorMoved { position, .. } => {}
        _ => (),
      },
      event::Event::DeviceEvent { event, .. } => match event {
        DeviceEvent::MouseMotion { delta } => {
          emit_listener(&mut self.mouse_motion_listeners, s, renderer);
        }
        _ => (),
      },
      event::Event::EventsCleared => {
        emit_listener(&mut self.events_cleared_listeners, s, renderer);
      }

      DeviceEvent => {}
    }
  }
}
