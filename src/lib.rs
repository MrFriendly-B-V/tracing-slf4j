//! # tracing-slf4j
//! SLF4j backend compatible with Rust's tracing crate.
//! Allows Java programs started from your program to log their logs to Rust's tracing.
//!
//! ## Usage
//! When using JNI's invocation API, the JAR file embedded in this crate
//! should be added to the classpath:
//!
//! 1. Save the jarfile (const `tracing_slf4j::DEPENDENCIES`) to disk
//! 2. Add the option `-Djava.class.path=<PATH TO JARFILE>` to the JVM's start parameters.
//!
//! After the JVM has been started, the setup `register_log_fn` function should be called:
//! ```ignore
//! tracing_slf4j::register_log_fn(&mut env).unwrap();
//! ```
//! This function will register the Rust logging handler with the JVM.

use jni::errors::Result;
use jni::objects::{JClass, JString};
use jni::strings::JNIStr;
use jni::sys::jint;
use jni::{JNIEnv, NativeMethod};
use std::ffi::{c_void, CString};
use tracing::{debug, error, info, trace, warn};

/// Java side of the 'bridge'
pub const DEPENDENCIES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/dependencies.jar"));

#[no_mangle]
extern "system" fn tracing_slf4j_impl(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    level: jint,
    string: JString<'_>,
) {
    let javastr = match env.get_string(&string) {
        Ok(s) => s,
        Err(_) => return,
    };

    let string = match javastr.to_str() {
        Ok(s) => s,
        Err(_) => return,
    };

    match level {
        0 => error!("{string}"),
        1 => warn!("{string}"),
        2 => info!("{string}"),
        3 => debug!("{string}"),
        _ => trace!("{string}"),
    }
}

/// Registry the native logging function with the JVM so it can be called by the logging framework.
///
/// # Errors
///
/// If registering the native method fails
pub fn register_log_fn(env: &mut JNIEnv<'_>) -> Result<()> {
    let logger_class = env.find_class("nl/mrfriendly/tracing/TracingSlf4jImpl")?;

    let fn_name = CString::new("tracingSlf4jImpl").unwrap();
    let fn_sig = CString::new("(ILjava/lang/String;)V").unwrap();

    env.register_native_methods(
        logger_class,
        &[NativeMethod {
            name: unsafe { JNIStr::from_ptr(fn_name.as_ptr()) }.to_owned(),
            fn_ptr: tracing_slf4j_impl as *mut c_void,
            sig: unsafe { JNIStr::from_ptr(fn_sig.as_ptr()) }.to_owned(),
        }],
    )?;

    Ok(())
}
