use std::ops::Deref;

use crate::native::jvmti_native::jmethodID;

use super::native::JavaMethod;

pub struct MethodId {
    pub native_id: JavaMethod,
}

impl Deref for MethodId {
    type Target = jmethodID;

    fn deref(&self) -> &Self::Target {
        &self.native_id
    }
}

pub struct Method {
    pub id: MethodId,
}

pub struct MethodSignature {
    pub name: String,
    pub signature: String,
}

impl MethodSignature {
    pub fn new(raw_signature: String, signature: String) -> MethodSignature {
        MethodSignature {
            name: raw_signature,
            signature,
        }
    }

    pub fn unknown() -> MethodSignature {
        MethodSignature {
            name: "<UNKNOWN METHOD>".to_string(),
            signature: "<UNKNOWN METHOD>".to_string(),
        }
    }
}
