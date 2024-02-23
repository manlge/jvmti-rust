use crate::native::jvmti_native::{jmethodID, jobject, jthread};

use self::jni::{JNIEnvironment, JNI};
use self::jvmti::{JVMTIEnvironment, JVMTI};
use super::capabilities::Capabilities;
use super::class::{ClassId, ClassSignature};
use super::error::NativeError;
use super::event::{EventCallbacks, VMEvent};
use super::mem::MemoryAllocation;
use super::method::{MethodId, MethodSignature};
use super::native::{JavaObject, JavaThread};
use super::thread::Thread;
use super::version::VersionNumber;

pub mod jni;
pub mod jvm;
pub mod jvmti;

/// `Environment` combines the functionality of both `JNI` and `JVMTI` by wrapping an instance of
/// both and delegating the method calls to their corresponding recipients.
pub struct Environment {
    jvmti: JVMTIEnvironment,
    jni: JNIEnvironment,
}

impl Environment {
    pub fn new(jvmti: JVMTIEnvironment, jni: JNIEnvironment) -> Environment {
        Environment {
            jvmti: jvmti,
            jni: jni,
        }
    }
}

impl JVMTI for Environment {
    fn get_version_number(&self) -> VersionNumber {
        self.jvmti.get_version_number()
    }

    fn add_capabilities(
        &mut self,
        new_capabilities: &Capabilities,
    ) -> Result<Capabilities, NativeError> {
        self.jvmti.add_capabilities(new_capabilities)
    }

    fn get_capabilities(&self) -> Capabilities {
        self.jvmti.get_capabilities()
    }

    fn set_event_callbacks(&mut self, callbacks: EventCallbacks) -> Option<NativeError> {
        self.jvmti.set_event_callbacks(callbacks)
    }

    fn set_event_notification_mode(&mut self, event: VMEvent, mode: bool) -> Option<NativeError> {
        self.jvmti.set_event_notification_mode(event, mode)
    }

    fn get_thread_info(&self, thread_id: &jthread) -> Result<Thread, NativeError> {
        self.jvmti.get_thread_info(thread_id)
    }

    fn get_method_declaring_class(&self, method_id: &MethodId) -> Result<ClassId, NativeError> {
        self.jvmti.get_method_declaring_class(method_id)
    }

    fn get_method_name(&self, method_id: jmethodID) -> Result<MethodSignature, NativeError> {
        self.jvmti.get_method_name(method_id)
    }

    fn get_class_signature(&self, class_id: &ClassId) -> Result<ClassSignature, NativeError> {
        self.jvmti.get_class_signature(class_id)
    }

    fn allocate(&self, len: usize) -> Result<MemoryAllocation, NativeError> {
        self.jvmti.allocate(len)
    }

    fn deallocate(&self) {
        self.jvmti.deallocate()
    }

    fn get_all_threads(&self) -> Result<Vec<jthread>, NativeError> {
        self.jvmti.get_all_threads()
    }

    fn run_agent_thread(
        &self,
        thread: crate::native::jvmti_native::jthread,
        proc: crate::native::jvmti_native::jvmtiStartFunction,
        arg: *const std::os::raw::c_void,
        priority: crate::native::jvmti_native::jint,
    ) -> Result<(), NativeError> {
        self.jvmti.run_agent_thread(thread, proc, arg, priority)
    }

    fn get_stack_trace(
        &self,
        thread: crate::native::jvmti_native::jthread,
    ) -> Result<(), NativeError> {
        self.jvmti.get_stack_trace(thread)
    }

    fn get_local_object(
        &self,
        thread: crate::native::jvmti_native::jthread,
        depth: crate::native::jvmti_native::jint,
        slot: crate::native::jvmti_native::jint,
    ) -> Result<jobject, NativeError> {
        self.jvmti.get_local_object(thread, depth, slot)
    }

    fn get_thread_state(&self, thread: jthread) -> Result<u32, NativeError> {
        self.jvmti.get_thread_state(thread)
    }

    fn add_to_bootstrap_classloader_search(&self, class_path: &str) -> Result<(), NativeError> {
        self.jvmti.add_to_bootstrap_classloader_search(class_path)
    }

    fn raw_monitor_enter(
        &self,
        monitor: crate::native::jvmti_native::jrawMonitorID,
    ) -> Result<(), NativeError> {
        self.jvmti.raw_monitor_enter(monitor)
    }

    fn raw_monitor_exit(
        &self,
        monitor: crate::native::jvmti_native::jrawMonitorID,
    ) -> Result<(), NativeError> {
        self.jvmti.raw_monitor_exit(monitor)
    }

    fn create_raw_monitor(
        &self,
        name: &str,
    ) -> Result<crate::native::jvmti_native::jrawMonitorID, NativeError> {
        self.jvmti.create_raw_monitor(name)
    }

    fn destroy_raw_monitor(
        &self,
        monitor: crate::native::jvmti_native::jrawMonitorID,
    ) -> Result<(), NativeError> {
        self.jvmti.destroy_raw_monitor(monitor)
    }

    fn retransform_classes(
        &self,
        count: crate::native::jvmti_native::jint,
        class: *const crate::native::jvmti_native::jclass,
    ) -> Result<(), NativeError> {
        self.jvmti.retransform_classes(count, class)
    }
}

impl JNI for Environment {
    fn get_object_class(&self, object_id: &JavaObject) -> ClassId {
        self.jni.get_object_class(object_id)
    }

    fn find_class(&self, clazz: &str) -> ClassId {
        self.jni.find_class(clazz)
    }

    fn get_method_id(&self, class: &ClassId, name: &str, sig: &str) -> MethodId {
        self.jni.get_method_id(class, name, sig)
    }

    fn new_object_a(&self, class: &ClassId, method: &MethodId, arg: jobject) -> JavaObject {
        self.jni.new_object_a(class, method, arg)
    }

    fn is_instance_of(
        &self,
        object: crate::native::jvmti_native::jobject,
        class: crate::native::jvmti_native::jclass,
    ) -> bool {
        self.jni.is_instance_of(object, class)
    }

    fn call_static_boolean_method(
        &self,
        class: crate::native::jvmti_native::jclass,
        method: crate::native::jvmti_native::jmethodID,
    ) -> bool {
        self.jni.call_static_boolean_method(class, method)
    }

    fn get_static_method_id(&self, class: &ClassId, name: &str, sig: &str) -> jmethodID {
        self.jni.get_static_method_id(class, name, sig)
    }

    fn new_string_utf(&self, str: &str) -> crate::native::jvmti_native::jstring {
        self.jni.new_string_utf(str)
    }

    fn is_assignable_from(
        &self,
        sub: crate::native::jvmti_native::jclass,
        sup: crate::native::jvmti_native::jclass,
    ) -> bool {
        self.jni.is_assignable_from(sub, sup)
    }

    fn call_static_object_method(
        &self,
        class: crate::native::jvmti_native::jclass,
        method: jmethodID,
    ) -> jobject {
        self.jni.call_static_object_method(class, method)
    }

    fn get_string_utf_chars(&self, str: crate::native::jvmti_native::jstring) -> String {
        self.jni.get_string_utf_chars(str)
    }

    fn release_string_utf_chars(
        &self,
        str: crate::native::jvmti_native::jstring,
        chars: *const i8,
    ) {
        self.jni.release_string_utf_chars(str, chars)
    }

    fn call_long_method(
        &self,
        class: crate::native::jvmti_native::jclass,
        method: jmethodID,
    ) -> crate::native::jvmti_native::jlong {
        self.jni.call_long_method(class, method)
    }

    fn del_local_ref(&self, obj: jobject) {
        self.jni.del_local_ref(obj)
    }

    fn get_int_field(
        &self,
        obj: jobject,
        field: crate::native::jvmti_native::jfieldID,
    ) -> crate::native::jvmti_native::jint {
        self.jni.get_int_field(obj, field)
    }

    fn get_object_field(
        &self,
        obj: jobject,
        field: crate::native::jvmti_native::jfieldID,
    ) -> jobject {
        self.jni.get_object_field(obj, field)
    }

    fn get_field_id(
        &self,
        class: crate::native::jvmti_native::jclass,
        name: &str,
        sig: &str,
    ) -> crate::native::jvmti_native::jfieldID {
        self.jni.get_field_id(class, name, sig)
    }
}
