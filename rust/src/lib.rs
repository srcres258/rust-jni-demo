
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jint, jstring};

fn create_rust_string(src: &str) -> String {
    format!("Rust-created string, {}", src)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_top_srcres_apps_rustjnidemo_App_hello<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>, input: JString<'a>
) -> jstring {
    let input: String = env.get_string(&input).expect("Failed to get Java string.").into();
    let output = env.new_string(create_rust_string(&input)).expect("Failed to create Rust string.");
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_top_srcres_apps_rustjnidemo_App_helloFromTestStringField<'a>(
    mut env: JNIEnv<'a>, class: JClass<'a>
) -> jstring {
    let testStringObj = env.get_static_field(&class, "testString", "Ljava/lang/String;")
        .expect("Failed to get static field testString")
        .l()
        .expect("Failed to convert testString into JObject");
    let testString: String = env.get_string(&JString::from(testStringObj))
        .expect("Failed to get the value of testString")
        .into();
    let output = env.new_string(create_rust_string(&testString)).expect("Failed to create Rust string.");
    output.into_raw()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_top_srcres_apps_rustjnidemo_App_modifyTestStringFromRust<'a>(
    mut env: JNIEnv<'a>, class: JClass<'a>, input: JString<'a>
) {
    let inputStr: String = env.get_string(&input).expect("Failed to receive the argument: input").into();
    let testStringFromRust = env.new_string(create_rust_string(&inputStr)).expect("Failed to create Rust string.");
    let testStringFromRustObj = JObject::from(testStringFromRust);
    let testStringFromRustId = env.get_static_field_id(&class, "testStringFromRust", "Ljava/lang/String;")
        .expect("Failed to get the ID of static field testStringFromRust");
    env.set_static_field(&class, &testStringFromRustId, JValue::from(&testStringFromRustObj))
        .expect("Failed to set static field testStringFromRust");
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_top_srcres_apps_rustjnidemo_App_actCallFromRust<'a>(
    mut env: JNIEnv<'a>, class: JClass<'a>, input: JString<'a>
) -> jstring {
    let inputStr: String = env.get_string(&input).expect("Failed to receive the argument: input").into();
    let testStringFromRust = env.new_string(create_rust_string(&inputStr)).expect("Failed to create Rust string.");
    let testStringFromRustObj = JObject::from(testStringFromRust);
    let callFromRustResult = env.call_static_method(&class, "callFromRust", "(Ljava/lang/String;)Ljava/lang/String;", &[JValue::from(&testStringFromRustObj)])
        .expect("Failed to invoke static method callFromRust");
    let callFromRustResultObj = callFromRustResult.l().expect("Failed to convert the method result into JObject.");
    JString::from(callFromRustResultObj).into_raw()
}
