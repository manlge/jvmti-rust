use environment::{jni::JNI, jvmti::JVMTI, Environment};
use native::{jvmti_native::jvmtiThreadInfo, JavaThread};
use thread::{Thread, ThreadId};

extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate time;
extern crate toml;
#[macro_use]
extern crate serde_derive;

pub mod agent;
pub mod bytecode;
pub mod capabilities;
pub mod class;
pub mod config;
pub mod context;
pub mod emulator;
pub mod environment;
pub mod error;
pub mod event;
pub mod event_handler;
pub mod instrumentation;
pub mod mem;
pub mod method;
pub mod native;
pub mod options;
pub mod runtime;
pub mod thread;
pub mod util;
pub mod version;

use util::stringify;

impl jvmtiThreadInfo {
    pub fn into_thread(self, env: &Environment, thread: JavaThread) -> Thread {
        let thread = Thread {
            id: ThreadId::new(thread),
            name: stringify(self.name),
            priority: self.priority as u32,
            is_daemon: self.is_daemon > 0,
        };
        env.deallocate(self.name as _).unwrap();
        if !self.thread_group.is_null() {
            env.delete_local_ref(&self.thread_group).unwrap();
        }
        if !self.context_class_loader.is_null() {
            env.delete_local_ref(&self.context_class_loader).unwrap();
        }
        thread
    }
}
