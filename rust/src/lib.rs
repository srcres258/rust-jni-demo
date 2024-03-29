
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};

fn create_rust_string(src: &str) -> String {
    format!("Rust-created string, {}", src)
}

fn create_rust_jstring<'a>(env: &'a mut JNIEnv, src: &'a str) -> JString<'a> {
    env.new_string(create_rust_string(src)).expect("Failed to create Rust string.")
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_top_srcres_apps_rustjnidemo_App_hello<'a>(
    mut env: JNIEnv<'a>, _: JClass<'a>, input: JString<'a>
) -> jstring {
    let input: String = env.get_string(&input).expect("Failed to get Java string.").into();
    let output = create_rust_jstring(&mut env, &input);
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
    let output = create_rust_jstring(&mut env, &testString);
    output.into_raw()
}
