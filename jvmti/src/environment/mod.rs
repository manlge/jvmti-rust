use std::os::raw::c_void;

use crate::native::{jvmti_native::*, JavaClass, JavaMethod, JavaObjectArray};

use self::jvmti::{JVMTIEnvironment, JVMTI};
use self::{
    jni::{JNIEnvironment, JNIError, JNI},
    jvmti::JVMTIError,
};
use super::capabilities::Capabilities;
use super::class::{ClassId, ClassSignature};
use super::error::NativeError;
use super::event::{EventCallbacks, VMEvent};
use super::mem::MemoryAllocation;
use super::method::{MethodId, MethodSignature};
use super::native::JavaObject;
use super::version::VersionNumber;

pub mod jni;
pub mod jvm;
pub mod jvmti;

/// `Environment` combines the functionality of both `JNI` and `JVMTI` by wrapping an instance of
/// both and delegating the method calls to their corresponding recipients.
pub struct Environment {
    jvmti: Box<dyn JVMTI>,
    jni: Box<dyn JNI>,
}

impl Environment {
    pub fn new(jvmti: JVMTIEnvironment, jni: JNIEnvironment) -> Environment {
        Environment {
            jvmti: Box::new(jvmti),
            jni: Box::new(jni),
        }
    }

    pub fn with_boxed(jvmti: Box<dyn JVMTI>, jni: Box<dyn JNI>) -> Environment {
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

    fn get_thread_info(&self, thread_id: &jthread) -> Result<jvmtiThreadInfo, NativeError> {
        self.jvmti.get_thread_info(thread_id)
    }

    fn get_method_declaring_class(&self, method_id: &JavaMethod) -> Result<ClassId, NativeError> {
        self.jvmti.get_method_declaring_class(method_id)
    }

    fn get_method_name(&self, method_id: JavaMethod) -> Result<MethodSignature, NativeError> {
        self.jvmti.get_method_name(method_id)
    }

    fn get_class_signature(&self, class_id: &jclass) -> Result<ClassSignature, NativeError> {
        self.jvmti.get_class_signature(class_id)
    }

    fn allocate(&self, len: usize) -> Result<MemoryAllocation, NativeError> {
        self.jvmti.allocate(len)
    }

    fn deallocate(&self, mem: *mut u8) -> Result<(), NativeError> {
        self.jvmti.deallocate(mem)
    }

    fn get_all_threads(&self) -> Result<&[jthread], NativeError> {
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
    ) -> Result<&[jvmtiFrameInfo], NativeError> {
        self.jvmti.get_stack_trace(thread)
    }

    fn get_local_object(
        &self,
        thread: crate::native::jvmti_native::jthread,
        depth: crate::native::jvmti_native::jint,
        slot: crate::native::jvmti_native::jint,
    ) -> Result<JavaObject, NativeError> {
        self.jvmti.get_local_object(thread, depth, slot)
    }

    fn get_thread_state(&self, thread: jthread) -> Result<u32, NativeError> {
        self.jvmti.get_thread_state(thread)
    }

    fn add_to_bootstrap_classloader_search(&self, class_path: &str) -> Result<(), NativeError> {
        self.jvmti.add_to_bootstrap_classloader_search(class_path)
    }

    fn raw_monitor_enter(&self, monitor: &jrawMonitorID) -> Result<(), NativeError> {
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

    fn iterate_over_instances_of_class(
        &self,
        klass: crate::native::jvmti_native::jclass,
        object_filter: crate::native::jvmti_native::jvmtiHeapObjectFilter,
        heap_object_callback: crate::native::jvmti_native::jvmtiHeapObjectCallback,
        user_data: *const std::os::raw::c_void,
    ) -> Result<(), NativeError> {
        self.jvmti.iterate_over_instances_of_class(
            klass,
            object_filter,
            heap_object_callback,
            user_data,
        )
    }

    fn get_objects_with_tags(&self, tags_list: &[jlong]) -> Result<Option<&[jobject]>, JVMTIError> {
        self.jvmti.get_objects_with_tags(tags_list)
    }

    fn iterate_over_heap(
        &self,
        object_filter: crate::native::jvmti_native::jvmtiHeapObjectFilter,
        heap_object_callback: crate::native::jvmti_native::jvmtiHeapObjectCallback,
        user_data: *const c_void,
    ) -> Result<(), NativeError> {
        self.jvmti
            .iterate_over_heap(object_filter, heap_object_callback, user_data)
    }

    fn get_current_thread(&self) -> Result<jthread, NativeError> {
        self.jvmti.get_current_thread()
    }

    fn get_classloader(&self, klass: &jclass) -> Result<JavaObject, NativeError> {
        self.jvmti.get_classloader(klass)
    }

    fn get_object_size(&self, object: &JavaObject) -> Result<jlong, NativeError> {
        self.jvmti.get_object_size(object)
    }

    fn get_loaded_classes(&self) -> Result<&[jclass], NativeError> {
        self.jvmti.get_loaded_classes()
    }

    fn get_class_loader_classes(
        &self,
        initiating_loader: &JavaObject,
    ) -> Result<&[crate::native::jvmti_native::jclass], NativeError> {
        self.jvmti.get_class_loader_classes(initiating_loader)
    }

    fn is_array_class(&self, class: &JavaClass) -> Result<bool, NativeError> {
        self.jvmti.is_array_class(class)
    }

    fn force_garbage_collection(&self) -> Result<(), NativeError> {
        self.jvmti.force_garbage_collection()
    }

    fn iterate_over_objects_reachable_from_object(
        &self,
        object: &jobject,
        callbck: jvmtiObjectReferenceCallback,
        user_data: *const c_void,
    ) -> Result<(), NativeError> {
        self.jvmti
            .iterate_over_objects_reachable_from_object(object, callbck, user_data)
    }

    fn get_object_hash_code(&self, object: &jobject) -> Result<jint, NativeError> {
        self.jvmti.get_object_hash_code(object)
    }

    fn follow_references(
        &self,
        heap_filter: jint,
        klass: &JavaClass,
        initial_object: &JavaObject,
        callbacks: *const jvmtiHeapCallbacks,
        user_data: *const c_void,
    ) {
        self.jvmti
            .follow_references(heap_filter, klass, initial_object, callbacks, user_data);
    }
}

impl JNI for Environment {
    fn get_object_class(&self, object_id: &JavaObject) -> Result<JavaClass, JNIError> {
        self.jni.get_object_class(object_id)
    }

    fn find_class(&self, clazz: &str) -> Result<ClassId, JNIError> {
        self.jni.find_class(clazz)
    }

    fn get_method(&self, class: &jclass, name: &str, sig: &str) -> Result<MethodId, JNIError> {
        self.jni.get_method(class, name, sig)
    }

    fn new_object(
        &self,
        class: &jclass,
        method: &JavaMethod,
        args: &[jvalue],
    ) -> Result<JavaObject, JNIError> {
        self.jni.new_object(class, method, args)
    }

    fn is_instance_of(&self, object: &JavaObject, class: &jclass) -> Result<bool, JNIError> {
        self.jni.is_instance_of(object, class)
    }

    fn call_static_boolean_method(
        &self,
        class: &jclass,
        method: &JavaMethod,
        args: &[jvalue],
    ) -> Result<bool, JNIError> {
        self.jni.call_static_boolean_method(class, method, args)
    }

    fn get_static_method(
        &self,
        class: &JavaClass,
        name: &str,
        sig: &str,
    ) -> Result<MethodId, JNIError> {
        self.jni.get_static_method(class, name, sig)
    }

    fn new_string_utf(&self, str: &str) -> Result<jstring, JNIError> {
        self.jni.new_string_utf(str)
    }

    fn is_assignable_from(&self, sub: &jclass, sup: &jclass) -> Result<bool, JNIError> {
        self.jni.is_assignable_from(sub, sup)
    }

    fn call_static_object_method(
        &self,
        class: &jclass,
        method: &JavaMethod,
        args: &[jvalue],
    ) -> Result<JavaObject, JNIError> {
        self.jni.call_static_object_method(class, method, args)
    }

    fn get_string_utf_chars(&self, string: &jstring) -> Result<String, JNIError> {
        self.jni.get_string_utf_chars(string)
    }

    fn release_string_utf_chars(&self, str: &jstring, chars: *const i8) -> Result<(), JNIError> {
        self.jni.release_string_utf_chars(str, chars)
    }

    fn call_long_method(
        &self,
        class: &JavaObject,
        method: &JavaMethod,
        args: &[jvalue],
    ) -> Result<jlong, JNIError> {
        self.jni.call_long_method(class, method, args)
    }

    fn delete_local_ref(&self, object: &JavaObject) -> Result<(), JNIError> {
        self.jni.delete_local_ref(object)
    }

    fn get_int_field(&self, obj: &JavaObject, field: &jfieldID) -> Result<jint, JNIError> {
        self.jni.get_int_field(obj, field)
    }

    fn get_object_field(&self, obj: &JavaObject, field: &jfieldID) -> Result<JavaObject, JNIError> {
        self.jni.get_object_field(obj, field)
    }

    fn get_field_id(&self, class: &jclass, name: &str, sig: &str) -> Result<jfieldID, JNIError> {
        self.jni.get_field_id(class, name, sig)
    }

    fn call_object_method(
        &self,
        object: &JavaObject,
        method: &JavaMethod,
        args: &[jvalue],
    ) -> Result<JavaObject, JNIError> {
        self.jni.call_object_method(object, method, args)
    }

    fn new_global_ref(&self, object: &JavaObject) -> Result<JavaObject, JNIError> {
        self.jni.new_global_ref(object)
    }

    fn delete_global_ref(&self, object: &JavaObject) -> Result<(), JNIError> {
        self.jni.delete_global_ref(object)
    }

    fn get_array_length(&self, array: &jarray) -> Result<jsize, JNIError> {
        self.jni.get_array_length(array)
    }

    fn get_object_array_element(
        &self,
        array: &JavaObjectArray,
        index: jsize,
    ) -> Result<JavaObject, JNIError> {
        self.jni.get_object_array_element(array, index)
    }
}
