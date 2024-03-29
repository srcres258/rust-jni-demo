
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_top_srcres_apps_rustjnidemo_App_hello<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>, input: JString<'a>) -> jstring {
    let input: String = env.get_string(&input).expect("Failed to get Java string.").into();
    let output = env.new_string(format!("Rust-created string, {}", input)).expect("Failed to create Rust string.");
    output.into_raw()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_top_srcres_apps_rustjnidemo_App_helloInt<'a>(
    _: JNIEnv<'a>, _: JClass<'a>, input: jint) -> jint {
    input + 1919810
}
