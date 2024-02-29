use crate::{
    environment::Environment,
    native::jvmti_native::{jlong, jmethodID, jobject, jthread},
};

use super::capabilities::Capabilities;
use super::class::{ClassId, ClassSignature};
use super::environment::jvm::JVMF;
use super::environment::jvmti::JVMTI;
use super::error::NativeError;
use super::event::{EventCallbacks, VMEvent};
use super::mem::MemoryAllocation;
use super::method::{MethodId, MethodSignature};
use super::native::JavaThread;
use super::runtime::*;
use super::thread::Thread;
use super::version::VersionNumber;
use std::collections::HashMap;

/// Allows testing of JVM and JVMTI-related functions by emulating (mocking) a JVM agent.
pub struct JVMEmulator {
    pub capabilities: Capabilities,
    pub callbacks: EventCallbacks,
    pub events: HashMap<VMEvent, bool>,
}

impl JVMEmulator {
    pub fn new() -> JVMEmulator {
        JVMEmulator {
            capabilities: Capabilities::new(),
            callbacks: EventCallbacks::new(),
            events: HashMap::new(),
        }
    }

    pub fn emit_method_entry(&self, env: Environment, event: MethodInvocationEvent) {
        match self.callbacks.method_entry {
            Some(handler) => {
                handler(env, event);
            }
            _ => (),
        }
    }
}

impl JVMF for JVMEmulator {
    fn get_environment(&self) -> Result<Box<JVMTI>, NativeError> {
        Ok(Box::new(JVMEmulator::new()))
    }

    fn destroy(&self) -> Result<(), NativeError> {
        Ok(())
    }

    fn attach_current_thread(&self) -> Result<(), NativeError> {
        Ok(())
    }
}

impl JVMTI for JVMEmulator {
    fn get_version_number(&self) -> VersionNumber {
        VersionNumber::unknown()
    }

    fn add_capabilities(
        &mut self,
        new_capabilities: &Capabilities,
    ) -> Result<Capabilities, NativeError> {
        let merged = self.capabilities.merge(&new_capabilities);
        self.capabilities = merged;
        Ok(self.capabilities.clone())
    }

    fn get_capabilities(&self) -> Capabilities {
        self.capabilities.clone()
    }

    fn set_event_callbacks(&mut self, callbacks: EventCallbacks) -> Option<NativeError> {
        self.callbacks = callbacks;

        None
    }

    fn set_event_notification_mode(&mut self, event: VMEvent, mode: bool) -> Option<NativeError> {
        self.events.insert(event, mode);
        None
    }

    fn get_thread_info(&self, thread_id: &JavaThread) -> Result<Thread, NativeError> {
        match *thread_id as u64 {
            _ => Err(NativeError::NotImplemented),
        }
    }

    fn get_method_declaring_class(&self, method_id: &MethodId) -> Result<ClassId, NativeError> {
        match method_id.native_id as u64 {
            _ => Err(NativeError::NotImplemented),
        }
    }

    fn get_method_name(&self, method_id: jmethodID) -> Result<MethodSignature, NativeError> {
        match method_id as u64 {
            0x01 => Ok(MethodSignature::new("".to_string(), "".to_string())),
            _ => Err(NativeError::NotImplemented),
        }
    }

    fn get_class_signature(&self, class_id: &ClassId) -> Result<ClassSignature, NativeError> {
        match class_id.native_id as u64 {
            _ => Err(NativeError::NotImplemented),
        }
    }

    fn allocate(&self, len: usize) -> Result<MemoryAllocation, NativeError> {
        Ok(MemoryAllocation {
            ptr: ::std::ptr::null_mut(),
            len: len,
        })
    }

    fn deallocate(&self) {}

    fn get_all_threads(&self) -> Result<Vec<jthread>, NativeError> {
        unimplemented!()
    }

    fn run_agent_thread(
        &self,
        thread: crate::native::jvmti_native::jthread,
        proc: crate::native::jvmti_native::jvmtiStartFunction,
        arg: *const std::os::raw::c_void,
        priority: crate::native::jvmti_native::jint,
    ) -> Result<(), NativeError> {
        Ok(())
    }

    fn get_stack_trace(
        &self,
        thread: crate::native::jvmti_native::jthread,
    ) -> Result<(), NativeError> {
        Ok(())
    }

    fn get_local_object(
        &self,
        thread: crate::native::jvmti_native::jthread,
        depth: crate::native::jvmti_native::jint,
        slot: crate::native::jvmti_native::jint,
    ) -> Result<crate::native::jvmti_native::jobject, NativeError> {
        unimplemented!()
    }

    fn get_thread_state(&self, thread: jthread) -> Result<u32, NativeError> {
        unimplemented!()
    }

    fn add_to_bootstrap_classloader_search(&self, class_path: &str) -> Result<(), NativeError> {
        unimplemented!()
    }

    fn raw_monitor_enter(
        &self,
        monitor: crate::native::jvmti_native::jrawMonitorID,
    ) -> Result<(), NativeError> {
        todo!()
    }

    fn raw_monitor_exit(
        &self,
        monitor: crate::native::jvmti_native::jrawMonitorID,
    ) -> Result<(), NativeError> {
        todo!()
    }

    fn create_raw_monitor(
        &self,
        name: &str,
    ) -> Result<crate::native::jvmti_native::jrawMonitorID, NativeError> {
        todo!()
    }

    fn destroy_raw_monitor(
        &self,
        monitor: crate::native::jvmti_native::jrawMonitorID,
    ) -> Result<(), NativeError> {
        todo!()
    }

    fn retransform_classes(
        &self,
        count: crate::native::jvmti_native::jint,
        class: *const crate::native::jvmti_native::jclass,
    ) -> Result<(), NativeError> {
        todo!()
    }

    fn iterate_over_instances_of_class(
        &self,
        klass: crate::native::jvmti_native::jclass,
        object_filter: crate::native::jvmti_native::jvmtiHeapObjectFilter,
        heap_object_callback: crate::native::jvmti_native::jvmtiHeapObjectCallback,
        user_data: *const std::os::raw::c_void,
    ) -> Result<(), NativeError> {
        todo!()
    }

    fn get_object_with_tag(&self, tags_list: &[jlong]) -> Result<&[jobject], NativeError> {
        todo!()
    }

    fn iterate_over_heap(
        &self,
        object_filter: crate::native::jvmti_native::jvmtiHeapObjectFilter,
        heap_object_callback: crate::native::jvmti_native::jvmtiHeapObjectCallback,
        user_data: *const std::os::raw::c_void,
    ) -> Result<(), NativeError> {
        todo!()
    }
}
