
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_top_srcres_apps_rustjnidemo_App_hello<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>, input: JString<'a>
) -> jstring {
    let input: String = env.get_string(&input).expect("Failed to get Java string.").into();
    let output = env.new_string(format!("Rust-created string, {}", input)).expect("Failed to create Rust string.");
    output.into_raw()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_top_srcres_apps_rustjnidemo_App_helloInt<'a>(
    _: JNIEnv<'a>, _: JClass<'a>, input: jint
) -> jint {
    input + 1919810
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_top_srcres_apps_rustjnidemo_App_helloFromTestIntField<'a>(
    mut env: JNIEnv<'a>, class: JClass<'a>
) -> jint {
    let testInt = env.get_static_field(&class, "testInt", "I")
        .expect("Failed to get static field testInt")
        .i()
        .expect("Failed to convert testInt into jint");
    testInt + 114
}
