
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

#[no_mangle]
pub extern "system" fn Java_top_srcres_apps_rustjnidemo_App_hello<'a>(mut env: JNIEnv<'a>,
                                                      _: JClass<'a>,
                                                      input: JString<'a>)
    -> jstring {
    let input: String = env.get_string(&input).expect("Failed to get Java string.").into();
    let output = env.new_string(format!("Rust-created string, {}", input)).expect("Failed to create Rust string.");
    output.into_raw()
}
