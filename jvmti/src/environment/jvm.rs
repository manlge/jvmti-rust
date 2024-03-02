use crate::{
    native::{
        jvmti_native::{jint, JavaVMAttachArgs},
        JNIEnvPtr,
    },
    options::Options,
};

use super::super::native::jvmti_native::JVMTI_VERSION;
use super::super::native::{JVMTIEnvPtr, JavaVMPtr};
use super::{
    super::environment::jvmti::{JVMTIEnvironment, JVMTI},
    jni::JNI,
};
use super::{
    super::error::{wrap_error, NativeError},
    jni::JNIEnvironment,
};
use libc::c_void;
use std::{ffi::CString, ptr};

pub const JNI_VERSION_1_6: jint = 0x00010006;

pub trait JVMF {
    fn get_environment(&self) -> Result<Box<JVMTI>, NativeError>;
    fn destroy(&self) -> Result<(), NativeError>;
    fn attach_current_thread(&self, thread_name: &str) -> Result<Box<dyn JNI>, NativeError>;
}
///
/// `JVMAgent` represents a binding to the JVM.
///
pub struct JVMAgent {
    vm: JavaVMPtr,
}

impl JVMAgent {
    /// Create a new `JVMAgent` instance
    pub fn new(vm: JavaVMPtr) -> JVMAgent {
        JVMAgent { vm: vm }
    }
}

impl JVMF for JVMAgent {
    /// Return the native JVMTI environment if available (ie. the current thread is attached to it)
    /// otherwise return an error message.
    fn get_environment(&self) -> Result<Box<JVMTI>, NativeError> {
        unsafe {
            let mut void_ptr: *mut c_void = ptr::null_mut() as *mut c_void;
            let penv_ptr: *mut *mut c_void = &mut void_ptr as *mut *mut c_void;
            let result =
                wrap_error((**self.vm).GetEnv.unwrap()(self.vm, penv_ptr, JVMTI_VERSION) as u32);

            match result {
                NativeError::NoError => {
                    let env_ptr: JVMTIEnvPtr = *penv_ptr as JVMTIEnvPtr;
                    let env = JVMTIEnvironment::new(env_ptr);
                    return Result::Ok(Box::new(env));
                }
                err @ _ => Result::Err(wrap_error(err as u32)),
            }
        }
    }

    fn destroy(&self) -> Result<(), NativeError> {
        unsafe {
            let error = (**self.vm).DestroyJavaVM.unwrap()(self.vm) as u32;

            if error == 0 {
                Ok(())
            } else {
                Err(wrap_error(error))
            }
        }
    }

    fn attach_current_thread(&self, thread_name: &str) -> Result<Box<dyn JNI>, NativeError> {
        let thread_name = CString::new(thread_name).unwrap();
        unsafe {
            let mut env = ptr::null_mut();
            let mut args: JavaVMAttachArgs = JavaVMAttachArgs {
                version: JNI_VERSION_1_6,
                name: thread_name.as_ptr() as *mut _,
                group: std::ptr::null_mut(),
            };

            let error = (**self.vm).AttachCurrentThread.unwrap()(
                self.vm,
                &mut env,
                &mut args as *const _ as *mut _,
            ) as u32;

            if error == 0 {
                Ok(Box::new(JNIEnvironment::new(env as JNIEnvPtr)))
            } else {
                Err(wrap_error(error))
            }
        }
    }
}
