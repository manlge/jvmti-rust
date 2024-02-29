use std::ffi::CString;

use crate::{
    method::MethodId,
    native::jvmti_native::{
        jboolean, jclass, jfieldID, jint, jlong, jmethodID, jobject, jstring, jvalue, va_list,
    },
    util::stringify,
};

use super::super::class::ClassId;
use super::super::native::{JNIEnvPtr, JavaObject};

///
/// `JNI` defines a set of operatations the JVM offers through it's JNI interface.
///
pub trait JNI {
    /// Return an `ClassId` belonging to the given Java object instance.
    fn get_object_class(&self, object_id: &JavaObject) -> ClassId;
    fn find_class(&self, clazz: &str) -> ClassId;
    fn get_method_id(&self, class: &ClassId, name: &str, sig: &str) -> MethodId;
    fn get_field_id(&self, class: jclass, name: &str, sig: &str) -> jfieldID;
    fn get_static_method_id(&self, class: &ClassId, name: &str, sig: &str) -> jmethodID;
    fn new_string_utf(&self, str: &str) -> jstring;
    fn get_string_utf_chars(&self, str: jstring) -> String;
    fn release_string_utf_chars(&self, str: jstring, chars: *const i8);
    fn new_object_a(&self, class: &ClassId, method: &MethodId, arg: jobject) -> JavaObject;
    fn is_instance_of(&self, object: jobject, class: jclass) -> bool;
    fn is_assignable_from(&self, sub: jclass, sup: jclass) -> bool;
    fn call_static_boolean_method(&self, class: jclass, method: jmethodID) -> bool;
    fn call_static_object_method(&self, class: jclass, method: jmethodID) -> jobject;
    fn call_long_method(&self, class: jclass, method: jmethodID) -> jlong;
    fn call_object_method(&self, class: jclass, method: jmethodID) -> jobject;
    fn del_local_ref(&self, obj: jobject);
    fn get_int_field(&self, obj: jobject, field: jfieldID) -> jint;
    fn get_object_field(&self, obj: jobject, field: jfieldID) -> jobject;
}

///
/// This is the native implementation of the `JNI` trait. Each trait method call is delegated
/// to the represented JNI instance.
pub struct JNIEnvironment {
    jni: JNIEnvPtr,
}

impl JNIEnvironment {
    pub fn new(jni: JNIEnvPtr) -> JNIEnvironment {
        JNIEnvironment { jni: jni }
    }
}

impl JNI for JNIEnvironment {
    fn get_object_class(&self, object_id: &JavaObject) -> ClassId {
        unsafe {
            let class_id = (**self.jni).GetObjectClass.unwrap()(self.jni, *object_id);

            ClassId {
                native_id: class_id,
            }
        }
    }

    fn find_class(&self, clazz: &str) -> ClassId {
        let cla_name = CString::new(clazz).unwrap();
        unsafe {
            let class_id = (**self.jni).FindClass.unwrap()(self.jni, cla_name.as_ptr());
            ClassId {
                native_id: class_id,
            }
        }
    }

    fn get_method_id(&self, class: &ClassId, name: &str, sig: &str) -> MethodId {
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();

        unsafe {
            let id: jmethodID = (**self.jni).GetMethodID.unwrap()(
                self.jni,
                class.native_id,
                name.as_ptr(),
                sig.as_ptr(),
            );
            MethodId { native_id: id }
        }
    }

    fn get_field_id(&self, class: jclass, name: &str, sig: &str) -> jfieldID {
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();

        unsafe { (**self.jni).GetFieldID.unwrap()(self.jni, class, name.as_ptr(), sig.as_ptr()) }
    }

    fn get_int_field(&self, obj: jobject, field: jfieldID) -> jint {
        unsafe {
            let value: jint = (**self.jni).GetIntField.unwrap()(self.jni, obj, field);
            value
        }
    }

    fn get_object_field(&self, obj: jobject, field: jfieldID) -> jobject {
        unsafe { (**self.jni).GetObjectField.unwrap()(self.jni, obj, field) }
    }

    fn get_static_method_id(&self, class: &ClassId, name: &str, sig: &str) -> jmethodID {
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();

        unsafe {
            let id: jmethodID = (**self.jni).GetStaticMethodID.unwrap()(
                self.jni,
                class.native_id,
                name.as_ptr(),
                sig.as_ptr(),
            );
            id
        }
    }

    fn new_string_utf(&self, str: &str) -> jstring {
        let str = CString::new(str).unwrap();
        unsafe {
            let id = (**self.jni).NewStringUTF.unwrap()(self.jni, str.as_ptr());
            id
        }
    }

    fn new_object_a(&self, class: &ClassId, method: &MethodId, arg: jobject) -> jobject {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(arg);
            let bindgen_data_: [u64; 1usize] = ::std::mem::transmute(raw.offset(0));
            let value = jvalue {
                _bindgen_data_: bindgen_data_,
            };

            let id = (**self.jni).NewObjectA.unwrap()(
                self.jni,
                class.native_id,
                method.native_id,
                &value,
            );
            id
        }
    }

    fn is_instance_of(&self, object: jobject, class: jclass) -> bool {
        unsafe { (**self.jni).IsInstanceOf.unwrap()(self.jni, object, class) == 1 }
    }

    fn is_assignable_from(&self, sub: jclass, sup: jclass) -> bool {
        unsafe { (**self.jni).IsAssignableFrom.unwrap()(self.jni, sub, sup) == 1 }
    }

    fn call_static_boolean_method(&self, class: jclass, method: jmethodID) -> bool {
        unsafe { (**self.jni).CallStaticBooleanMethod.unwrap()(self.jni, class, method) == 1 }
    }

    fn call_static_object_method(&self, class: jclass, method: jmethodID) -> jobject {
        unsafe { (**self.jni).CallStaticObjectMethod.unwrap()(self.jni, class, method) }
    }

    fn get_string_utf_chars(&self, str: jstring) -> String {
        let mut is_copy: jboolean = 1;
        unsafe {
            let chars = (**self.jni).GetStringUTFChars.unwrap()(self.jni, str, &mut is_copy);
            let ret = stringify(chars);
            self.release_string_utf_chars(str, chars);
            ret
        }
    }

    fn release_string_utf_chars(&self, str: jstring, chars: *const i8) {
        unsafe {
            (**self.jni).ReleaseStringUTFChars.unwrap()(self.jni, str, chars);
        }
    }

    fn call_long_method(&self, obj: jobject, method: jmethodID) -> jlong {
        unsafe { (**self.jni).CallLongMethod.unwrap()(self.jni, obj, method) }
    }

    fn call_object_method(&self, obj: jobject, method: jmethodID) -> jobject {
        unsafe { (**self.jni).CallObjectMethod.unwrap()(self.jni, obj, method) }
    }

    fn del_local_ref(&self, obj: jobject) {
        unsafe { (**self.jni).DeleteLocalRef.unwrap()(self.jni, obj) }
    }
}
