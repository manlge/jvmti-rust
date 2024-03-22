use std::ffi::CString;

use crate::{
    method::MethodId,
    native::jvmti_native::{
        jboolean, jbyte, jclass, jfieldID, jint, jlong, jmethodID, jobject, jstring, *,
    },
    util::stringify,
};

use super::super::class::ClassId;
use super::super::native::{JNIEnvPtr, JavaObject};

pub const TRUE: jboolean = 1;
pub const FALSE: jboolean = 0;

#[derive(Debug)]
pub enum JNIError {
    ClassNotFound(String),
    MethodNotFound(String, String),
    FieldNotFound(String),
    ObjectIsNull,
    ClassObjectIsNull,
}

impl From<jint> for jvalue {
    fn from(value: jint) -> Self {
        unsafe {
            let mut target_value: jvalue = Default::default();
            *target_value.i() = value;
            target_value
        }
    }
}

impl From<jlong> for jvalue {
    fn from(value: jlong) -> Self {
        unsafe {
            let mut target_value: jvalue = Default::default();
            *target_value.j() = value;
            target_value
        }
    }
}

impl From<jobject> for jvalue {
    fn from(value: jobject) -> Self {
        unsafe {
            let mut target_value: jvalue = Default::default();
            *target_value.l() = value;
            target_value
        }
    }
}

impl From<jboolean> for jvalue {
    fn from(value: jboolean) -> Self {
        unsafe {
            let mut target_value: jvalue = Default::default();
            *target_value.z() = value;
            target_value
        }
    }
}

impl From<jbyte> for jvalue {
    fn from(value: jbyte) -> Self {
        unsafe {
            let mut target_value: jvalue = Default::default();
            *target_value.b() = value;
            target_value
        }
    }
}

impl From<jchar> for jvalue {
    fn from(value: jchar) -> Self {
        unsafe {
            let mut target_value: jvalue = Default::default();
            *target_value.c() = value;
            target_value
        }
    }
}

impl From<jshort> for jvalue {
    fn from(value: jshort) -> Self {
        unsafe {
            let mut target_value: jvalue = Default::default();
            *target_value.s() = value;
            target_value
        }
    }
}

impl From<jfloat> for jvalue {
    fn from(value: jfloat) -> Self {
        unsafe {
            let mut target_value: jvalue = Default::default();
            *target_value.f() = value;
            target_value
        }
    }
}

impl From<jdouble> for jvalue {
    fn from(value: jdouble) -> Self {
        unsafe {
            let mut target_value: jvalue = Default::default();
            *target_value.d() = value;
            target_value
        }
    }
}

impl jvalue {
    pub fn null() -> jvalue {
        (std::ptr::null() as *const u8 as jobject).into()
    }
}

///
/// `JNI` defines a set of operatations the JVM offers through it's JNI interface.
///
pub trait JNI {
    /// Return an `ClassId` belonging to the given Java object instance.
    fn get_object_class(&self, object_id: &JavaObject) -> ClassId;
    fn find_class(&self, clazz: &str) -> Result<ClassId, JNIError>;
    fn get_method(&self, class: &jclass, name: &str, sig: &str) -> Result<MethodId, JNIError>;
    fn get_static_method(
        &self,
        class: &ClassId,
        name: &str,
        sig: &str,
    ) -> Result<MethodId, JNIError>;
    fn get_field_id(&self, class: jclass, name: &str, sig: &str) -> jfieldID;
    fn new_string_utf(&self, str: &str) -> jstring;
    fn get_string_utf_chars(&self, string: &jstring) -> Result<String, JNIError>;
    fn release_string_utf_chars(&self, str: jstring, chars: *const i8);
    fn new_object(&self, class: &ClassId, method: &MethodId, args: &[jvalue]) -> JavaObject;
    fn new_global_ref(&self, object: &jobject) -> jobject;
    fn delete_global_ref(&self, object: &jobject) -> Result<(), JNIError>;

    fn is_instance_of(&self, object: jobject, class: jclass) -> bool;
    fn is_assignable_from(&self, sub: &jclass, sup: &jclass) -> Result<bool, JNIError>;
    fn call_static_boolean_method(&self, class: jclass, method: jmethodID, args: &[jvalue])
        -> bool;
    fn call_static_object_method(
        &self,
        class: jclass,
        method: jmethodID,
        args: &[jvalue],
    ) -> jobject;
    fn call_long_method(
        &self,
        object: &jobject,
        method: &jmethodID,
        args: &[jvalue],
    ) -> Result<jlong, JNIError>;
    fn call_object_method(
        &self,
        object: &jobject,
        method: &jmethodID,
        args: &[jvalue],
    ) -> Result<jobject, JNIError>;
    fn delete_local_ref(&self, obj: &jobject);
    fn get_int_field(&self, obj: jobject, field: jfieldID) -> jint;
    fn get_object_field(&self, obj: jobject, field: jfieldID) -> jobject;
    fn get_array_length(&self, array: &jarray) -> Result<jsize, JNIError>;
    fn get_object_array_element(&self, array: jobjectArray, index: jsize) -> jobject;
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

    fn find_class(&self, clazz: &str) -> Result<ClassId, JNIError> {
        let cla_name = CString::new(clazz).unwrap();
        unsafe {
            let class_id = (**self.jni).FindClass.unwrap()(self.jni, cla_name.as_ptr());
            if class_id.is_null() {
                Err(JNIError::ClassNotFound(clazz.to_string()))
            } else {
                Ok(ClassId {
                    native_id: class_id,
                })
            }
        }
    }

    fn get_method(
        &self,
        class: &jclass,
        method_name: &str,
        signature: &str,
    ) -> Result<MethodId, JNIError> {
        if class.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        let name = CString::new(method_name).unwrap();
        let sig = CString::new(signature).unwrap();

        unsafe {
            let id: jmethodID =
                (**self.jni).GetMethodID.unwrap()(self.jni, *class, name.as_ptr(), sig.as_ptr());
            if id.is_null() {
                Err(JNIError::MethodNotFound(
                    method_name.to_string(),
                    signature.to_string(),
                ))
            } else {
                Ok(MethodId { native_id: id })
            }
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

    fn get_static_method(
        &self,
        class: &ClassId,
        method_name: &str,
        signature: &str,
    ) -> Result<MethodId, JNIError> {
        let name = CString::new(method_name).unwrap();
        let sig = CString::new(signature).unwrap();

        unsafe {
            let id: jmethodID = (**self.jni).GetStaticMethodID.unwrap()(
                self.jni,
                class.native_id,
                name.as_ptr(),
                sig.as_ptr(),
            );

            if id.is_null() {
                Err(JNIError::MethodNotFound(
                    method_name.to_string(),
                    signature.to_string(),
                ))
            } else {
                Ok(MethodId { native_id: id })
            }
        }
    }

    fn new_string_utf(&self, str: &str) -> jstring {
        let str = CString::new(str).unwrap();
        unsafe {
            let id = (**self.jni).NewStringUTF.unwrap()(self.jni, str.as_ptr());
            id
        }
    }

    fn new_object(&self, class: &ClassId, method: &MethodId, args: &[jvalue]) -> jobject {
        unsafe {
            let id = (**self.jni).NewObjectA.unwrap()(
                self.jni,
                class.native_id,
                method.native_id,
                args.as_ptr(),
            );
            id
        }
    }

    fn is_instance_of(&self, object: jobject, class: jclass) -> bool {
        unsafe { (**self.jni).IsInstanceOf.unwrap()(self.jni, object, class) == 1 }
    }

    fn is_assignable_from(&self, sub: &jclass, sup: &jclass) -> Result<bool, JNIError> {
        if sup.is_null() {
            return Err(JNIError::ClassObjectIsNull);
        }

        if sub.is_null() {
            return Err(JNIError::ClassObjectIsNull);
        }

        unsafe { Ok((**self.jni).IsAssignableFrom.unwrap()(self.jni, *sub, *sup) == 1) }
    }

    fn call_static_boolean_method(
        &self,
        class: jclass,
        method: jmethodID,
        args: &[jvalue],
    ) -> bool {
        unsafe {
            (**self.jni).CallStaticBooleanMethodA.unwrap()(self.jni, class, method, args.as_ptr())
                == 1
        }
    }

    fn call_static_object_method(
        &self,
        class: jclass,
        method: jmethodID,
        args: &[jvalue],
    ) -> jobject {
        unsafe {
            (**self.jni).CallStaticObjectMethodA.unwrap()(self.jni, class, method, args.as_ptr())
        }
    }

    fn get_string_utf_chars(&self, string: &jstring) -> Result<String, JNIError> {
        if string.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        let mut is_copy: jboolean = FALSE;
        unsafe {
            let chars = (**self.jni).GetStringUTFChars.unwrap()(self.jni, *string, &mut is_copy);
            let ret = stringify(chars);
            if !chars.is_null() {
                self.release_string_utf_chars(*string, chars);
            }
            Ok(ret)
        }
    }

    fn release_string_utf_chars(&self, str: jstring, chars: *const i8) {
        unsafe {
            (**self.jni).ReleaseStringUTFChars.unwrap()(self.jni, str, chars);
        }
    }

    fn call_long_method(
        &self,
        object: &jobject,
        method: &jmethodID,
        args: &[jvalue],
    ) -> Result<jlong, JNIError> {
        if object.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        unsafe {
            Ok((**self.jni).CallLongMethodA.unwrap()(
                self.jni,
                *object,
                *method,
                args.as_ptr(),
            ))
        }
    }

    fn call_object_method(
        &self,
        object: &jobject,
        method: &jmethodID,
        args: &[jvalue],
    ) -> Result<jobject, JNIError> {
        if object.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        unsafe {
            Ok((**self.jni).CallObjectMethodA.unwrap()(
                self.jni,
                *object,
                *method,
                args.as_ptr(),
            ))
        }
    }

    fn delete_local_ref(&self, obj: &jobject) {
        unsafe { (**self.jni).DeleteLocalRef.unwrap()(self.jni, *obj) }
    }

    fn new_global_ref(&self, object: &jobject) -> jobject {
        unsafe { (**self.jni).NewGlobalRef.unwrap()(self.jni, *object) }
    }

    fn delete_global_ref(&self, object: &jobject) -> Result<(), JNIError> {
        if object.is_null() {
            return Err(JNIError::ObjectIsNull);
        }

        unsafe { Ok((**self.jni).DeleteGlobalRef.unwrap()(self.jni, *object)) }
    }

    fn get_array_length(&self, array: &jarray) -> Result<jsize, JNIError> {
        if array.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        unsafe { Ok((**self.jni).GetArrayLength.unwrap()(self.jni, *array)) }
    }

    fn get_object_array_element(&self, array: jobjectArray, index: jsize) -> jobject {
        unsafe { (**self.jni).GetObjectArrayElement.unwrap()(self.jni, array, index) }
    }
}
