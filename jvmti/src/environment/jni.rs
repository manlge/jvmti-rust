use std::ffi::CString;

use crate::{
    method::MethodId,
    native::{jvmti_native::*, JavaArray, JavaClass, JavaMethod, *},
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
    MethodIsNull,
    FieldIsNull,
}

impl From<jint> for JavaValue {
    fn from(value: jint) -> Self {
        unsafe {
            let mut target_value: JavaValue = Default::default();
            *target_value.i() = value;
            target_value
        }
    }
}

impl From<jlong> for JavaValue {
    fn from(value: jlong) -> Self {
        unsafe {
            let mut target_value: JavaValue = Default::default();
            *target_value.j() = value;
            target_value
        }
    }
}

impl From<JavaObject> for JavaValue {
    fn from(value: JavaObject) -> Self {
        unsafe {
            let mut target_value: JavaValue = Default::default();
            *target_value.l() = value;
            target_value
        }
    }
}

impl From<jboolean> for JavaValue {
    fn from(value: jboolean) -> Self {
        unsafe {
            let mut target_value: JavaValue = Default::default();
            *target_value.z() = value;
            target_value
        }
    }
}

impl From<jbyte> for JavaValue {
    fn from(value: jbyte) -> Self {
        unsafe {
            let mut target_value: JavaValue = Default::default();
            *target_value.b() = value;
            target_value
        }
    }
}

impl From<jchar> for JavaValue {
    fn from(value: jchar) -> Self {
        unsafe {
            let mut target_value: JavaValue = Default::default();
            *target_value.c() = value;
            target_value
        }
    }
}

impl From<jshort> for JavaValue {
    fn from(value: jshort) -> Self {
        unsafe {
            let mut target_value: JavaValue = Default::default();
            *target_value.s() = value;
            target_value
        }
    }
}

impl From<jfloat> for JavaValue {
    fn from(value: jfloat) -> Self {
        unsafe {
            let mut target_value: JavaValue = Default::default();
            *target_value.f() = value;
            target_value
        }
    }
}

impl From<jdouble> for JavaValue {
    fn from(value: jdouble) -> Self {
        unsafe {
            let mut target_value: JavaValue = Default::default();
            *target_value.d() = value;
            target_value
        }
    }
}

impl JavaValue {
    pub fn null() -> JavaValue {
        (std::ptr::null() as *const u8 as JavaObject).into()
    }
}

///
/// `JNI` defines a set of operatations the JVM offers through it's JNI interface.
///
pub trait JNI {
    /// Return an `ClassId` belonging to the given Java object instance.
    fn get_object_class(&self, object: &JavaObject) -> Result<JavaClass, JNIError>;
    fn find_class(&self, clazz: &str) -> Result<ClassId, JNIError>;
    fn get_method(&self, class: &JavaClass, name: &str, sig: &str) -> Result<MethodId, JNIError>;
    fn get_static_method(
        &self,
        class: &JavaClass,
        name: &str,
        sig: &str,
    ) -> Result<MethodId, JNIError>;
    fn get_field_id(&self, class: &JavaClass, name: &str, sig: &str)
        -> Result<JavaField, JNIError>;
    fn new_string_utf(&self, str: &str) -> Result<JavaString, JNIError>;
    fn get_string_utf_chars(&self, string: &JavaString) -> Result<String, JNIError>;
    fn release_string_utf_chars(&self, str: &JavaString, chars: *const i8) -> Result<(), JNIError>;
    fn new_object(
        &self,
        class: &JavaClass,
        method: &JavaMethod,
        args: &[JavaValue],
    ) -> Result<JavaObject, JNIError>;
    fn new_global_ref(&self, object: &JavaObject) -> Result<JavaObject, JNIError>;
    fn delete_global_ref(&self, object: &JavaObject) -> Result<(), JNIError>;
    fn is_instance_of(&self, object: &JavaObject, class: &JavaClass) -> Result<bool, JNIError>;
    fn is_assignable_from(&self, sub: &JavaClass, sup: &JavaClass) -> Result<bool, JNIError>;
    fn call_static_boolean_method(
        &self,
        class: &JavaClass,
        method: &JavaMethod,
        args: &[JavaValue],
    ) -> Result<bool, JNIError>;
    fn call_static_object_method(
        &self,
        class: &JavaClass,
        method: &JavaMethod,
        args: &[JavaValue],
    ) -> Result<JavaObject, JNIError>;
    fn call_long_method(
        &self,
        object: &JavaObject,
        method: &JavaMethod,
        args: &[JavaValue],
    ) -> Result<jlong, JNIError>;
    fn call_object_method(
        &self,
        object: &JavaObject,
        method: &JavaMethod,
        args: &[JavaValue],
    ) -> Result<JavaObject, JNIError>;
    fn delete_local_ref(&self, obj: &JavaObject) -> Result<(), JNIError>;
    fn get_int_field(&self, obj: &JavaObject, field: &JavaField) -> Result<jint, JNIError>;
    fn get_object_field(&self, obj: &JavaObject, field: &JavaField)
        -> Result<JavaObject, JNIError>;
    fn get_array_length(&self, array: &JavaArray) -> Result<jsize, JNIError>;
    fn get_object_array_element(
        &self,
        array: &JavaObjectArray,
        index: jsize,
    ) -> Result<JavaObject, JNIError>;
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
    fn get_object_class(&self, object: &JavaObject) -> Result<JavaClass, JNIError> {
        if object.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        unsafe { Ok((**self.jni).GetObjectClass.unwrap()(self.jni, *object)) }
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
        class: &JavaClass,
        method_name: &str,
        signature: &str,
    ) -> Result<MethodId, JNIError> {
        if class.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        let name = CString::new(method_name).unwrap();
        let sig = CString::new(signature).unwrap();

        unsafe {
            let id: JavaMethod =
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

    fn get_field_id(
        &self,
        class: &JavaClass,
        name: &str,
        sig: &str,
    ) -> Result<JavaField, JNIError> {
        if class.is_null() {
            return Err(JNIError::ClassObjectIsNull);
        }
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();

        Ok(unsafe {
            (**self.jni).GetFieldID.unwrap()(self.jni, *class, name.as_ptr(), sig.as_ptr())
        })
    }

    fn get_int_field(&self, object: &JavaObject, field: &JavaField) -> Result<jint, JNIError> {
        if object.is_null() {
            return Err(JNIError::ObjectIsNull);
        }

        if field.is_null() {
            return Err(JNIError::FieldIsNull);
        }
        unsafe { Ok((**self.jni).GetIntField.unwrap()(self.jni, *object, *field)) }
    }

    fn get_object_field(
        &self,
        object: &JavaObject,
        field: &JavaField,
    ) -> Result<JavaObject, JNIError> {
        if object.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        Ok(unsafe { (**self.jni).GetObjectField.unwrap()(self.jni, *object, *field) })
    }

    fn get_static_method(
        &self,
        class: &JavaClass,
        method_name: &str,
        signature: &str,
    ) -> Result<MethodId, JNIError> {
        let name = CString::new(method_name).unwrap();
        let sig = CString::new(signature).unwrap();

        unsafe {
            let id: JavaMethod = (**self.jni).GetStaticMethodID.unwrap()(
                self.jni,
                *class,
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

    fn new_string_utf(&self, str: &str) -> Result<JavaString, JNIError> {
        let str = CString::new(str).unwrap();
        unsafe { Ok((**self.jni).NewStringUTF.unwrap()(self.jni, str.as_ptr())) }
    }

    fn new_object(
        &self,
        class: &JavaClass,
        method: &JavaMethod,
        args: &[JavaValue],
    ) -> Result<JavaObject, JNIError> {
        if class.is_null() {
            return Err(JNIError::ClassObjectIsNull);
        }
        if method.is_null() {
            return Err(JNIError::MethodIsNull);
        }
        Ok(unsafe {
            let id = (**self.jni).NewObjectA.unwrap()(self.jni, *class, *method, args.as_ptr());
            id
        })
    }

    fn is_instance_of(&self, object: &JavaObject, class: &JavaClass) -> Result<bool, JNIError> {
        if object.is_null() {
            return Err(JNIError::ObjectIsNull);
        }

        if class.is_null() {
            return Err(JNIError::ClassObjectIsNull);
        }
        Ok(unsafe { (**self.jni).IsInstanceOf.unwrap()(self.jni, *object, *class) == 1 })
    }

    fn is_assignable_from(&self, sub: &JavaClass, sup: &JavaClass) -> Result<bool, JNIError> {
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
        class: &JavaClass,
        method: &JavaMethod,
        args: &[JavaValue],
    ) -> Result<bool, JNIError> {
        if class.is_null() {
            return Err(JNIError::ClassObjectIsNull);
        }

        if method.is_null() {
            return Err(JNIError::MethodIsNull);
        }
        Ok(unsafe {
            (**self.jni).CallStaticBooleanMethodA.unwrap()(self.jni, *class, *method, args.as_ptr())
                == 1
        })
    }

    fn call_static_object_method(
        &self,
        class: &JavaClass,
        method: &JavaMethod,
        args: &[JavaValue],
    ) -> Result<JavaObject, JNIError> {
        if class.is_null() {
            return Err(JNIError::ClassObjectIsNull);
        }

        if method.is_null() {
            return Err(JNIError::MethodIsNull);
        }
        Ok(unsafe {
            (**self.jni).CallStaticObjectMethodA.unwrap()(self.jni, *class, *method, args.as_ptr())
        })
    }

    fn get_string_utf_chars(&self, string: &JavaString) -> Result<String, JNIError> {
        if string.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        let mut is_copy: jboolean = FALSE;
        unsafe {
            let chars = (**self.jni).GetStringUTFChars.unwrap()(self.jni, *string, &mut is_copy);
            let ret = stringify(chars);
            if !chars.is_null() {
                self.release_string_utf_chars(string, chars).unwrap();
            }
            Ok(ret)
        }
    }

    fn release_string_utf_chars(&self, str: &JavaString, chars: *const i8) -> Result<(), JNIError> {
        if str.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        unsafe {
            (**self.jni).ReleaseStringUTFChars.unwrap()(self.jni, *str, chars);
        }
        Ok(())
    }

    fn call_long_method(
        &self,
        object: &JavaObject,
        method: &JavaMethod,
        args: &[JavaValue],
    ) -> Result<jlong, JNIError> {
        if object.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        if method.is_null() {
            return Err(JNIError::MethodIsNull);
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
        object: &JavaObject,
        method: &JavaMethod,
        args: &[JavaValue],
    ) -> Result<JavaObject, JNIError> {
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

    fn delete_local_ref(&self, object: &JavaObject) -> Result<(), JNIError> {
        if object.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        unsafe { (**self.jni).DeleteLocalRef.unwrap()(self.jni, *object) }
        Ok(())
    }

    fn new_global_ref(&self, object: &JavaObject) -> Result<JavaObject, JNIError> {
        if object.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        Ok(unsafe { (**self.jni).NewGlobalRef.unwrap()(self.jni, *object) })
    }

    fn delete_global_ref(&self, object: &JavaObject) -> Result<(), JNIError> {
        if object.is_null() {
            return Err(JNIError::ObjectIsNull);
        }

        unsafe { Ok((**self.jni).DeleteGlobalRef.unwrap()(self.jni, *object)) }
    }

    fn get_array_length(&self, array: &JavaArray) -> Result<jsize, JNIError> {
        if array.is_null() {
            return Err(JNIError::ObjectIsNull);
        }
        unsafe { Ok((**self.jni).GetArrayLength.unwrap()(self.jni, *array)) }
    }

    fn get_object_array_element(
        &self,
        array: &JavaObjectArray,
        index: jsize,
    ) -> Result<JavaObject, JNIError> {
        Ok(unsafe { (**self.jni).GetObjectArrayElement.unwrap()(self.jni, *array, index) })
    }
}
