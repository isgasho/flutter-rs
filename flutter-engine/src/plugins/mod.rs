pub mod platform;
pub mod textinput;
pub mod dialog;

use super::{ffi, FlutterEngineInner};
use std::{
    ptr::null,
    mem,
    sync::Weak,
    collections::HashMap,
    ffi::CString,
    borrow::Cow,
    sync::Arc,
};

pub struct PluginRegistry {
    map: HashMap<String, Box<dyn Plugin>>,
    pub engine: Weak<FlutterEngineInner>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry {
            map: HashMap::new(),
            engine: Weak::new(),
        }
    }
    pub fn set_engine(&mut self, engine: Weak<FlutterEngineInner>) {
        self.engine = engine;
    }
    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        let name = {
            let name = plugin.init_channel(self as &PluginRegistry);
            name.to_owned()
        };
        self.map.insert(name, plugin);
    }
    pub fn handle(&mut self, msg: PlatformMessage, engine: Arc<FlutterEngineInner>, window: &mut glfw::Window) {
        for (channel, plugin) in &mut self.map {
            if channel == &msg.channel {
                info!("Processing message from channel: {}", channel);
                plugin.handle(&msg, engine.clone(), window);
            }
        }
    }
    pub fn get_plugin(&self, channel: &str) -> Option<&Box<dyn Plugin>> {
        self.map.get(channel)
    }
}

#[derive(Debug)]
pub struct PlatformMessage<'a, 'b> {
    pub channel: Cow<'a, str>,
    pub message: &'b [u8],
    pub response_handle: Option<&'a ffi::FlutterPlatformMessageResponseHandle>,
}

impl<'a, 'b> PlatformMessage<'a, 'b> {
    fn get_response_handle(&self) -> Option<usize> {
        self.response_handle.map(|r| {
            r as *const ffi::FlutterPlatformMessageResponseHandle as usize
        })
    }
}

impl<'a, 'b> Into<ffi::FlutterPlatformMessage> for &PlatformMessage<'a, 'b> {
    fn into(self) -> ffi::FlutterPlatformMessage {
        let channel = CString::new(&*self.channel).unwrap();
        let message_ptr = self.message.as_ptr();
        let message_len = self.message.len();

        let response_handle = if let Some(h) = self.response_handle {
            h as *const ffi::FlutterPlatformMessageResponseHandle
        } else {
            null()
        };

        ffi::FlutterPlatformMessage {
            struct_size: mem::size_of::<ffi::FlutterPlatformMessage>(),
            channel: channel.into_raw(),
            message: message_ptr,
            message_size: message_len,
            response_handle,
        }            
    }
}

pub trait Plugin {
    fn init_channel(&self, &PluginRegistry) -> &str;
    fn handle(&mut self, msg: &PlatformMessage, engine: Arc<FlutterEngineInner>, window: &mut glfw::Window);
}
