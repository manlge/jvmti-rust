const JVMTI_ERROR_INVALID_MONITOR: u32 = 50;
const JVMTI_ERROR_NOT_MONITOR_OWNER: u32 = 51;
const JVMTI_ERROR_ILLEGAL_ARGUMENT: u32 = 103;

/// A type-safe representation of possible errors
#[derive(Debug)]
pub enum NativeError {
    NoError = 0,
    MustPossessCapability = 99,
    NullPointer = 100,
    OutOfMemory = 110,
    NotEnabled = 111,
    NotAvailable = 112,
    UnexpectedInternalError = 113,
    ThreadNotAttached = 115,
    Disconnected = 116,
    NotImplemented = 999999, // <- now this is a "temporary" hack until the library is under heavy development
    UnknownError,
    ThreadNotAlive = 15,
    InvalidMonitor = JVMTI_ERROR_INVALID_MONITOR as isize,
    NotMonitorOwner = JVMTI_ERROR_NOT_MONITOR_OWNER as isize,
    IllegalArgument = JVMTI_ERROR_ILLEGAL_ARGUMENT as isize,
}

/// Turn a native error code into a type-safe error
pub fn wrap_error(code: u32) -> NativeError {
    match code {
        0 => NativeError::NoError,
        99 => NativeError::MustPossessCapability,
        100 => NativeError::NullPointer,
        110 => NativeError::OutOfMemory,
        111 => NativeError::NotEnabled,
        112 => NativeError::NotAvailable,
        113 => NativeError::UnexpectedInternalError,
        115 => NativeError::ThreadNotAttached,
        116 => NativeError::Disconnected,
        999999 => NativeError::NotImplemented,
        15 => NativeError::ThreadNotAlive,
        JVMTI_ERROR_INVALID_MONITOR => NativeError::InvalidMonitor,
        JVMTI_ERROR_NOT_MONITOR_OWNER => NativeError::NotMonitorOwner,
        JVMTI_ERROR_ILLEGAL_ARGUMENT => NativeError::IllegalArgument,
        _ => {
            eprintln!("Unknown error code was detected: {}", code);
            NativeError::UnknownError
        }
    }
}

impl std::fmt::Display for NativeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", translate_error(self))
    }
}

/// Turn native error codes into meaningful and user-readable strings
pub fn translate_error(code: &NativeError) -> String {
    match code {
        &NativeError::NoError => "No error has occurred.",
        &NativeError::MustPossessCapability => "The capability being used is false in this environment.",
        &NativeError::NullPointer => "Pointer is unexpectedly NULL.",
        &NativeError::OutOfMemory => "The function attempted to allocate memory and no more memory was available for allocation.",
        &NativeError::NotEnabled => "The desired functionality has not been enabled in this virtual machine.",
        &NativeError::NotAvailable => "The desired functionality is not available in the current phase. Always returned if the virtual machine has completed running.",
        &NativeError::UnexpectedInternalError => "An unexpected internal error has occurred.",
        &NativeError::ThreadNotAttached => "The thread being used to call this function is not attached to the virtual machine. Calls must be made from attached threads.",
        &NativeError::Disconnected => "The JVM TI environment provided is no longer connected or is not an environment.",
        &NativeError::NotImplemented => "This function is not implemented yet",
        &NativeError::UnknownError => "Unknown error.",
        &NativeError::ThreadNotAlive => "thread is not live (has not been started or is now dead).",
        &NativeError::InvalidMonitor => "Invalid raw monitor.",
        &NativeError::NotMonitorOwner => "This thread doesn't own the raw monitor.",
        &NativeError::IllegalArgument => "Illegal argument.",

    }.to_string()
}
