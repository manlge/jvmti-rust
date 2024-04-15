use crate::native::jvmti_native::*;

use super::super::error::{wrap_error, NativeError};
use super::super::event::{EventCallbacks, VMEvent};
use super::super::event_handler::*;
use super::super::mem::MemoryAllocation;
use super::super::method::MethodSignature;
use super::super::native::jvmti_native::jvmtiCapabilities;
use super::super::native::{
    JVMTIEnvPtr, JavaClass, JavaInstance, JavaLong, JavaObject, JavaThread, MutByteArray, MutString,
};
use super::super::util::stringify;
use super::super::version::VersionNumber;
use super::{super::capabilities::Capabilities, jni::FALSE};
use super::{
    super::class::{ClassId, ClassSignature, JavaType},
    jni::TRUE,
};

use std::{ffi::CString, os::raw::c_void, ptr};
#[derive(Debug)]
pub enum JVMTIError {
    NativeError(NativeError),
}

impl std::fmt::Display for JVMTIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Native Error {}", self.to_string())
    }
}

pub trait JVMTI {
    ///
    /// Return the JVM TI version number, which includes major, minor and micro version numbers.
    ///
    fn get_version_number(&self) -> VersionNumber;
    /// Set new capabilities by adding the capabilities whose values are set to true in new_caps.
    /// All previous capabilities are retained.
    /// Some virtual machines may allow a limited set of capabilities to be added in the live phase.
    fn add_capabilities(
        &mut self,
        new_capabilities: &Capabilities,
    ) -> Result<Capabilities, NativeError>;
    fn get_capabilities(&self) -> Capabilities;
    /// Set the functions to be called for each event. The callbacks are specified by supplying a
    /// replacement function table. The function table is copied--changes to the local copy of the
    /// table have no effect. This is an atomic action, all callbacks are set at once. No events
    /// are sent before this function is called. When an entry is None no event is sent.
    /// An event must be enabled and have a callback in order to be sent--the order in which this
    /// function and set_event_notification_mode are called does not affect the result.
    fn set_event_callbacks(&mut self, callbacks: EventCallbacks) -> Option<NativeError>;
    fn set_event_notification_mode(&mut self, event: VMEvent, mode: bool) -> Option<NativeError>;
    fn get_thread_info(&self, thread_id: &JavaThread) -> Result<jvmtiThreadInfo, NativeError>;
    fn get_method_declaring_class(&self, method_id: &jmethodID) -> Result<ClassId, NativeError>;
    fn get_method_name(&self, method_id: jmethodID) -> Result<MethodSignature, NativeError>;
    fn get_class_signature(&self, class: &jclass) -> Result<ClassSignature, NativeError>;
    fn allocate(&self, len: usize) -> Result<MemoryAllocation, NativeError>;
    fn deallocate(&self, mem: *mut u8) -> Result<(), NativeError>;
    fn get_all_threads(&self) -> Result<&[jthread], NativeError>;
    fn get_local_object(
        &self,
        thread: jthread,
        depth: jint,
        slot: jint,
    ) -> Result<jobject, NativeError>;
    fn run_agent_thread(
        &self,
        thread: jthread,
        proc: jvmtiStartFunction,
        arg: *const c_void,
        priority: jint,
    ) -> Result<(), NativeError>;
    fn get_current_thread(&self) -> Result<jthread, NativeError>;
    fn get_stack_trace(&self, thread: jthread) -> Result<&[jvmtiFrameInfo], NativeError>;
    fn get_thread_state(&self, thread: jthread) -> Result<u32, NativeError>;
    fn add_to_bootstrap_classloader_search(&self, class_path: &str) -> Result<(), NativeError>;
    fn raw_monitor_enter(&self, monitor: &jrawMonitorID) -> Result<(), NativeError>;
    fn raw_monitor_exit(&self, monitor: jrawMonitorID) -> Result<(), NativeError>;
    fn create_raw_monitor(&self, name: &str) -> Result<jrawMonitorID, NativeError>;
    fn destroy_raw_monitor(&self, monitor: jrawMonitorID) -> Result<(), NativeError>;
    fn retransform_classes(&self, count: jint, class: *const jclass) -> Result<(), NativeError>;
    fn iterate_over_heap(
        &self,
        object_filter: jvmtiHeapObjectFilter,
        heap_object_callback: jvmtiHeapObjectCallback,
        user_data: *const c_void,
    ) -> Result<(), NativeError>;
    fn iterate_over_instances_of_class(
        &self,
        klass: jclass,
        object_filter: jvmtiHeapObjectFilter,
        heap_object_callback: jvmtiHeapObjectCallback,
        user_data: *const c_void,
    ) -> Result<(), NativeError>;
    fn get_objects_with_tags(&self, tags_list: &[jlong]) -> Result<&[jobject], JVMTIError>;
    fn get_classloader(&self, klass: &jclass) -> Result<jobject, NativeError>;
    fn get_object_size(&self, object: &jobject) -> Result<jlong, NativeError>;
    fn get_object_hash_code(&self, object: &jobject) -> Result<jint, NativeError>;
    fn get_loaded_classes(&self) -> Result<&[jclass], NativeError>;
    fn get_class_loader_classes(
        &self,
        initiating_loader: &jobject,
    ) -> Result<&[jclass], NativeError>;
    fn is_array_class(&self, class: &JavaClass) -> Result<bool, NativeError>;
    fn force_garbage_collection(&self) -> Result<(), NativeError>;
    fn iterate_over_objects_reachable_from_object(
        &self,
        object: &jobject,
        callbck: jvmtiObjectReferenceCallback,
        user_data: *const c_void,
    ) -> Result<(), NativeError>;
}

pub struct JVMTIEnvironment {
    jvmti: JVMTIEnvPtr,
}

impl JVMTIEnvironment {
    pub fn new(env_ptr: JVMTIEnvPtr) -> JVMTIEnvironment {
        JVMTIEnvironment { jvmti: env_ptr }
    }
}

impl JVMTI for JVMTIEnvironment {
    fn get_version_number(&self) -> VersionNumber {
        unsafe {
            let mut version: i32 = 0;
            let version_ptr = &mut version;
            (**self.jvmti).GetVersionNumber.unwrap()(self.jvmti, version_ptr);
            let uversion = *version_ptr as u32;
            VersionNumber::from_u32(&uversion)
        }
    }

    fn add_capabilities(
        &mut self,
        new_capabilities: &Capabilities,
    ) -> Result<Capabilities, NativeError> {
        let native_caps = new_capabilities.to_native();
        let caps_ptr: *const jvmtiCapabilities = &native_caps;

        unsafe {
            match wrap_error((**self.jvmti).AddCapabilities.unwrap()(
                self.jvmti, caps_ptr,
            )) {
                NativeError::NoError => Ok(self.get_capabilities()),
                err @ _ => Err(err),
            }
        }
    }

    fn get_capabilities(&self) -> Capabilities {
        unsafe {
            let caps = Capabilities::new();
            let mut native_caps = caps.to_native();
            {
                let cap_ptr = &mut native_caps;
                (**self.jvmti).GetCapabilities.unwrap()(self.jvmti, cap_ptr);
            }
            Capabilities::from_native(&native_caps)
        }
    }

    fn set_event_callbacks(&mut self, callbacks: EventCallbacks) -> Option<NativeError> {
        register_vm_init_callback(callbacks.vm_init);
        register_vm_start_callback(callbacks.vm_start);
        register_vm_death_callback(callbacks.vm_death);
        register_vm_object_alloc_callback(callbacks.vm_object_alloc);
        register_method_entry_callback(callbacks.method_entry);
        register_method_exit_callback(callbacks.method_exit);
        register_thread_start_callback(callbacks.thread_start);
        register_thread_end_callback(callbacks.thread_end);
        register_exception_callback(callbacks.exception);
        register_exception_catch_callback(callbacks.exception_catch);
        register_monitor_wait_callback(callbacks.monitor_wait);
        register_monitor_waited_callback(callbacks.monitor_waited);
        register_monitor_contended_enter_callback(callbacks.monitor_contended_enter);
        register_monitor_contended_endered_callback(callbacks.monitor_contended_entered);
        register_field_access_callback(callbacks.field_access);
        register_field_modification_callback(callbacks.field_modification);
        register_garbage_collection_start(callbacks.garbage_collection_start);
        register_garbage_collection_finish(callbacks.garbage_collection_finish);
        register_class_file_load_hook(callbacks.class_file_load_hook);

        let (native_callbacks, callbacks_size) = registered_callbacks();

        unsafe {
            match wrap_error((**self.jvmti).SetEventCallbacks.unwrap()(
                self.jvmti,
                &native_callbacks,
                callbacks_size,
            )) {
                NativeError::NoError => None,
                err @ _ => Some(err),
            }
        }
    }

    fn set_event_notification_mode(&mut self, event: VMEvent, mode: bool) -> Option<NativeError> {
        unsafe {
            let mode_i = match mode {
                true => 1,
                false => 0,
            };
            let sptr: JavaObject = ptr::null_mut();

            match wrap_error((**self.jvmti).SetEventNotificationMode.unwrap()(
                self.jvmti,
                mode_i,
                event as u32,
                sptr,
            )) {
                NativeError::NoError => None,
                err @ _ => Some(err),
            }
        }
    }

    fn get_thread_info(&self, thread_id: &JavaThread) -> Result<jvmtiThreadInfo, NativeError> {
        let mut info = unsafe { std::mem::zeroed() };
        unsafe {
            match wrap_error((**self.jvmti).GetThreadInfo.unwrap()(
                self.jvmti, *thread_id, &mut info,
            )) {
                NativeError::NoError => Ok(info),
                err @ _ => Err(err),
            }
        }
    }

    fn get_thread_state(&self, thread: jthread) -> Result<u32, NativeError> {
        let mut state: jint = 0;
        unsafe {
            match wrap_error((**self.jvmti).GetThreadState.unwrap()(
                self.jvmti, thread, &mut state,
            )) {
                NativeError::NoError => Ok(state as u32 & JVMTI_JAVA_LANG_THREAD_STATE_MASK),
                err @ _ => Err(err),
            }
        }
    }

    fn get_method_declaring_class(&self, method: &jmethodID) -> Result<ClassId, NativeError> {
        let mut jstruct: JavaInstance = JavaInstance {
            _hacky_hack_workaround: 0,
        };
        let mut jclass_instance: JavaClass = &mut jstruct;
        let meta_ptr: *mut JavaClass = &mut jclass_instance;

        unsafe {
            match wrap_error((**self.jvmti).GetMethodDeclaringClass.unwrap()(
                self.jvmti, *method, meta_ptr,
            )) {
                NativeError::NoError => Ok(ClassId {
                    native_id: *meta_ptr,
                }),
                err @ _ => Err(err),
            }
        }
    }

    fn get_method_name(&self, method_id: jmethodID) -> Result<MethodSignature, NativeError> {
        let mut method_name = ptr::null_mut();
        let mut signature: MutString = ptr::null_mut();
        let mut generic_sig: MutString = ptr::null_mut();

        unsafe {
            match wrap_error((**self.jvmti).GetMethodName.unwrap()(
                self.jvmti,
                method_id,
                &mut method_name,
                &mut signature,
                &mut generic_sig,
            )) {
                NativeError::NoError => Ok(MethodSignature::new(
                    stringify(method_name),
                    stringify(signature),
                )),
                err @ _ => Err(err),
            }
        }
    }

    fn get_class_signature(&self, class: &jclass) -> Result<ClassSignature, NativeError> {
        unsafe {
            let mut generic: *mut i8 = ptr::null_mut();
            let mut signature: *mut i8 = ptr::null_mut();

            match wrap_error((**self.jvmti).GetClassSignature.unwrap()(
                self.jvmti,
                *class,
                &mut signature,
                &mut generic,
            )) {
                NativeError::NoError => {
                    let rsignature = stringify(signature);
                    let x = JavaType::parse(&rsignature.as_str()).unwrap();
                    (**self.jvmti).Deallocate.unwrap()(self.jvmti, signature as _);
                    (**self.jvmti).Deallocate.unwrap()(self.jvmti, generic as _);
                    Ok(ClassSignature::new(x, rsignature.clone()))
                }
                err @ _ => Err(err),
            }
        }
    }

    fn allocate(&self, len: usize) -> Result<MemoryAllocation, NativeError> {
        let size: JavaLong = len as JavaLong;
        let mut ptr: MutByteArray = ptr::null_mut();
        let mem_ptr: *mut MutByteArray = &mut ptr;

        unsafe {
            match wrap_error((**self.jvmti).Allocate.unwrap()(self.jvmti, size, mem_ptr)) {
                NativeError::NoError => Ok(MemoryAllocation { ptr: ptr, len: len }),
                err @ _ => Err(err),
            }
        }
    }

    fn deallocate(&self, mem: *mut u8) -> Result<(), NativeError> {
        unsafe {
            match wrap_error((**self.jvmti).Deallocate.unwrap()(self.jvmti, mem)) {
                NativeError::NoError => Ok(()),
                err @ _ => Err(err),
            }
        }
    }

    fn get_all_threads(&self) -> Result<&[jthread], NativeError> {
        let mut threads_count: jint = 0;
        let mut threads_ptr: *mut jthread = std::ptr::null_mut();

        unsafe {
            match wrap_error((**self.jvmti).GetAllThreads.unwrap()(
                self.jvmti,
                &mut threads_count,
                &mut threads_ptr,
            )) {
                NativeError::NoError => {
                    let threads = std::slice::from_raw_parts(threads_ptr, threads_count as usize);

                    Ok(threads)
                }
                err @ _ => Err(err),
            }
        }
    }

    fn run_agent_thread(
        &self,
        thread: jthread,
        proc: jvmtiStartFunction,
        arg: *const c_void,
        priority: jint,
    ) -> Result<(), NativeError> {
        unsafe {
            match wrap_error((**self.jvmti).RunAgentThread.unwrap()(
                self.jvmti, thread, proc, arg, priority,
            )) {
                NativeError::NoError => Ok(()),
                err @ _ => Err(err),
            }
        }
    }

    fn get_stack_trace(&self, thread: jthread) -> Result<&[jvmtiFrameInfo], NativeError> {
        unsafe {
            let mut count: jint = 0;
            let mut info = [jvmtiFrameInfo::default(); 1024];

            match wrap_error((**self.jvmti).GetStackTrace.unwrap()(
                self.jvmti,
                thread,
                0,
                info.len() as i32,
                info.as_mut_ptr(),
                &mut count,
            )) {
                NativeError::NoError => Ok(std::slice::from_raw_parts(
                    info.as_mut_ptr(),
                    count as usize,
                )),
                err @ _ => Err(err),
            }
        }
    }

    fn get_local_object(
        &self,
        thread: jthread,
        depth: jint,
        slot: jint,
    ) -> Result<jobject, NativeError> {
        unsafe {
            let mut value: JavaObject = std::mem::zeroed();
            match wrap_error((**self.jvmti).GetLocalObject.unwrap()(
                self.jvmti, thread, depth, slot, &mut value,
            )) {
                NativeError::NoError => Ok(value),
                err @ _ => Err(err),
            }
        }
    }

    fn add_to_bootstrap_classloader_search(&self, class_path: &str) -> Result<(), NativeError> {
        let path = CString::new(class_path).unwrap();
        unsafe {
            match wrap_error((**self.jvmti).AddToBootstrapClassLoaderSearch.unwrap()(
                self.jvmti,
                path.as_ptr(),
            )) {
                NativeError::NoError => Ok(()),
                err @ _ => Err(err),
            }
        }
    }

    fn raw_monitor_enter(&self, monitor: &jrawMonitorID) -> Result<(), NativeError> {
        unsafe {
            match wrap_error((**self.jvmti).RawMonitorEnter.unwrap()(
                self.jvmti, *monitor,
            )) {
                NativeError::NoError => Ok(()),
                err @ _ => Err(err),
            }
        }
    }

    fn raw_monitor_exit(&self, monitor: jrawMonitorID) -> Result<(), NativeError> {
        unsafe {
            match wrap_error((**self.jvmti).RawMonitorExit.unwrap()(self.jvmti, monitor)) {
                NativeError::NoError => Ok(()),
                err @ _ => Err(err),
            }
        }
    }

    fn create_raw_monitor(&self, name: &str) -> Result<jrawMonitorID, NativeError> {
        let mut monitor: jrawMonitorID = unsafe { std::mem::zeroed() };
        let monitor_ptr: *mut jrawMonitorID = &mut monitor;
        unsafe {
            let name = CString::new(name).unwrap();
            match wrap_error((**self.jvmti).CreateRawMonitor.unwrap()(
                self.jvmti,
                name.as_ptr(),
                monitor_ptr,
            )) {
                NativeError::NoError => Ok(monitor),
                err @ _ => Err(err),
            }
        }
    }

    fn destroy_raw_monitor(&self, monitor: jrawMonitorID) -> Result<(), NativeError> {
        unsafe {
            match wrap_error((**self.jvmti).DestroyRawMonitor.unwrap()(
                self.jvmti, monitor,
            )) {
                NativeError::NoError => Ok(()),
                err @ _ => Err(err),
            }
        }
    }

    fn retransform_classes(&self, count: jint, class: *const jclass) -> Result<(), NativeError> {
        unsafe {
            match wrap_error((**self.jvmti).RetransformClasses.unwrap()(
                self.jvmti, count, class,
            )) {
                NativeError::NoError => Ok(()),
                err @ _ => Err(err),
            }
        }
    }

    fn iterate_over_heap(
        &self,
        object_filter: jvmtiHeapObjectFilter,
        heap_object_callback: jvmtiHeapObjectCallback,
        user_data: *const c_void,
    ) -> Result<(), NativeError> {
        unsafe {
            match wrap_error((**self.jvmti).IterateOverHeap.unwrap()(
                self.jvmti,
                object_filter,
                heap_object_callback,
                user_data,
            )) {
                NativeError::NoError => Ok(()),
                err @ _ => Err(err),
            }
        }
    }

    fn iterate_over_instances_of_class(
        &self,
        klass: jclass,
        object_filter: jvmtiHeapObjectFilter,
        heap_object_callback: jvmtiHeapObjectCallback,
        user_data: *const c_void,
    ) -> Result<(), NativeError> {
        unsafe {
            match wrap_error((**self.jvmti).IterateOverInstancesOfClass.unwrap()(
                self.jvmti,
                klass,
                object_filter,
                heap_object_callback,
                user_data,
            )) {
                NativeError::NoError => Ok(()),
                err @ _ => Err(err),
            }
        }
    }

    fn get_objects_with_tags(&self, tags_list: &[jlong]) -> Result<&[JavaObject], JVMTIError> {
        let mut count: jint = 0;
        let mut object_result_ptr: *mut jobject = std::ptr::null_mut();
        // let mut tag_result_ptr: *mut jlong = std::ptr::null_mut();

        unsafe {
            match wrap_error((**self.jvmti).GetObjectsWithTags.unwrap()(
                self.jvmti,
                tags_list.len() as i32,
                tags_list.as_ptr(),
                &mut count,
                &mut object_result_ptr,
                std::ptr::null_mut(),
            )) {
                NativeError::NoError => {
                    let objects = std::slice::from_raw_parts(object_result_ptr, count as usize);
                    return Result::Ok(objects);
                }
                err => {
                    return Err(JVMTIError::NativeError(err));
                }
            }
        }
    }

    fn get_current_thread(&self) -> Result<jthread, NativeError> {
        let mut thread: jthread = unsafe { std::mem::zeroed() };
        unsafe {
            match wrap_error((**self.jvmti).GetCurrentThread.unwrap()(
                self.jvmti,
                &mut thread,
            )) {
                NativeError::NoError => Ok(thread),
                err @ _ => Err(err),
            }
        }
    }

    fn get_classloader(&self, klass: &jclass) -> Result<jobject, NativeError> {
        let mut classloader: jobject = unsafe { std::mem::zeroed() };
        unsafe {
            match wrap_error((**self.jvmti).GetClassLoader.unwrap()(
                self.jvmti,
                *klass,
                &mut classloader,
            )) {
                NativeError::NoError => Ok(classloader),
                err @ _ => Err(err),
            }
        }
    }

    fn get_object_size(&self, object: &jobject) -> Result<jlong, NativeError> {
        let mut size: jlong = 0;
        unsafe {
            match wrap_error((**self.jvmti).GetObjectSize.unwrap()(
                self.jvmti, *object, &mut size,
            )) {
                NativeError::NoError => Ok(size),
                err @ _ => Err(err),
            }
        }
    }

    fn get_object_hash_code(&self, object: &jobject) -> Result<jint, NativeError> {
        let mut hash_code: jint = 0;
        unsafe {
            match wrap_error((**self.jvmti).GetObjectHashCode.unwrap()(
                self.jvmti,
                *object,
                &mut hash_code,
            )) {
                NativeError::NoError => Ok(hash_code),
                err @ _ => Err(err),
            }
        }
    }

    fn get_loaded_classes(&self) -> Result<&[jclass], NativeError> {
        let mut count: jint = 0;
        let mut classes: *mut jclass = std::ptr::null_mut();
        unsafe {
            match wrap_error((**self.jvmti).GetLoadedClasses.unwrap()(
                self.jvmti,
                &mut count,
                &mut classes,
            )) {
                NativeError::NoError => Ok(std::slice::from_raw_parts(classes, count as usize)),
                err @ _ => Err(err),
            }
        }
    }

    fn get_class_loader_classes(
        &self,
        initiating_loader: &jobject,
    ) -> Result<&[jclass], NativeError> {
        let mut count: jint = 0;
        let mut classes: *mut jclass = std::ptr::null_mut();
        unsafe {
            match wrap_error((**self.jvmti).GetClassLoaderClasses.unwrap()(
                self.jvmti,
                *initiating_loader,
                &mut count,
                &mut classes,
            )) {
                NativeError::NoError => Ok(std::slice::from_raw_parts(classes, count as usize)),
                err @ _ => Err(err),
            }
        }
    }

    fn is_array_class(&self, class: &JavaClass) -> Result<bool, NativeError> {
        let mut is_array: jboolean = FALSE;
        unsafe {
            match wrap_error((**self.jvmti).IsArrayClass.unwrap()(
                self.jvmti,
                *class,
                &mut is_array,
            )) {
                NativeError::NoError => Ok(is_array == TRUE),
                err @ _ => Err(err),
            }
        }
    }

    fn force_garbage_collection(&self) -> Result<(), NativeError> {
        unsafe {
            match wrap_error((**self.jvmti).ForceGarbageCollection.unwrap()(self.jvmti)) {
                NativeError::NoError => Ok(()),
                err @ _ => Err(err),
            }
        }
    }

    fn iterate_over_objects_reachable_from_object(
        &self,
        object: &jobject,
        callbck: jvmtiObjectReferenceCallback,
        user_data: *const c_void,
    ) -> Result<(), NativeError> {
        unsafe {
            match wrap_error((**self.jvmti)
                .IterateOverObjectsReachableFromObject
                .unwrap()(
                self.jvmti, *object, callbck, user_data
            )) {
                NativeError::NoError => Ok(()),
                err => Err(err),
            }
        }
    }
}
