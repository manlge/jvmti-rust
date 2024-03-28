use crate::{
    environment::{jni::JNI, Environment},
    native::jvmti_native::{
        jclass, jlong, jmethodID, jobject, jthread, jvmtiFrameInfo, jvmtiThreadInfo,
    },
};

use super::capabilities::Capabilities;
use super::class::{ClassId, ClassSignature};
use super::environment::jvm::JVMF;
use super::environment::jvmti::JVMTI;
use super::error::NativeError;
use super::event::{EventCallbacks, VMEvent};
use super::mem::MemoryAllocation;
use super::method::MethodSignature;
use super::native::JavaThread;
use super::runtime::*;
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

    fn attach_current_thread(&self, thread_name: &str) -> Result<Box<dyn JNI>, NativeError> {
        unimplemented!()
    }

    fn get_jni_environment(&self) -> Result<Box<dyn JNI>, NativeError> {
        todo!()
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

    fn get_thread_info(&self, thread_id: &JavaThread) -> Result<jvmtiThreadInfo, NativeError> {
        match *thread_id as u64 {
            _ => Err(NativeError::NotImplemented),
        }
    }

    fn get_method_declaring_class(&self, method_id: &jmethodID) -> Result<ClassId, NativeError> {
        match *method_id as u64 {
            _ => Err(NativeError::NotImplemented),
        }
    }

    fn get_method_name(&self, method_id: jmethodID) -> Result<MethodSignature, NativeError> {
        match method_id as u64 {
            0x01 => Ok(MethodSignature::new("".to_string(), "".to_string())),
            _ => Err(NativeError::NotImplemented),
        }
    }

    fn get_class_signature(&self, class_id: &jclass) -> Result<ClassSignature, NativeError> {
        match *class_id as u64 {
            _ => Err(NativeError::NotImplemented),
        }
    }

    fn allocate(&self, len: usize) -> Result<MemoryAllocation, NativeError> {
        Ok(MemoryAllocation {
            ptr: ::std::ptr::null_mut(),
            len: len,
        })
    }

    fn deallocate(&self, mem: *mut u8) -> Result<(), NativeError> {
        unimplemented!()
    }

    fn get_all_threads(&self) -> Result<&[jthread], NativeError> {
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
    ) -> Result<&[jvmtiFrameInfo], NativeError> {
        unimplemented!()
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

    fn get_current_thread(&self) -> Result<jthread, NativeError> {
        todo!()
    }

    fn get_classloader(&self, klass: &jclass) -> Result<jobject, NativeError> {
        todo!()
    }

    fn get_object_size(&self, object: &jobject) -> Result<jlong, NativeError> {
        todo!()
    }

    fn get_loaded_classes(&self) -> Result<&[crate::native::jvmti_native::jclass], NativeError> {
        todo!()
    }

    fn get_class_loader_classes(
        &self,
        initiating_loader: &jobject,
    ) -> Result<&[crate::native::jvmti_native::jclass], NativeError> {
        todo!()
    }

    fn is_array_class(
        &self,
        class: crate::native::jvmti_native::jclass,
    ) -> Result<bool, NativeError> {
        todo!()
    }

    fn force_garbage_collection(&self) -> Result<(), NativeError> {
        todo!()
    }
}
