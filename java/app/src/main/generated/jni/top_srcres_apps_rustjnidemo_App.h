/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class top_srcres_apps_rustjnidemo_App */

#ifndef _Included_top_srcres_apps_rustjnidemo_App
#define _Included_top_srcres_apps_rustjnidemo_App
#ifdef __cplusplus
extern "C" {
#endif
/*
 * Class:     top_srcres_apps_rustjnidemo_App
 * Method:    hello
 * Signature: (Ljava/lang/String;)Ljava/lang/String;
 */
JNIEXPORT jstring JNICALL Java_top_srcres_apps_rustjnidemo_App_hello
  (JNIEnv *, jclass, jstring);

/*
 * Class:     top_srcres_apps_rustjnidemo_App
 * Method:    helloInt
 * Signature: (I)I
 */
JNIEXPORT jint JNICALL Java_top_srcres_apps_rustjnidemo_App_helloInt
  (JNIEnv *, jclass, jint);

/*
 * Class:     top_srcres_apps_rustjnidemo_App
 * Method:    helloFromTestIntField
 * Signature: ()I
 */
JNIEXPORT jint JNICALL Java_top_srcres_apps_rustjnidemo_App_helloFromTestIntField
  (JNIEnv *, jclass);

/*
 * Class:     top_srcres_apps_rustjnidemo_App
 * Method:    helloFromTestStringField
 * Signature: ()Ljava/lang/String;
 */
JNIEXPORT jstring JNICALL Java_top_srcres_apps_rustjnidemo_App_helloFromTestStringField
  (JNIEnv *, jclass);

/*
 * Class:     top_srcres_apps_rustjnidemo_App
 * Method:    modifyTestStringFromRust
 * Signature: (Ljava/lang/String;)V
 */
JNIEXPORT void JNICALL Java_top_srcres_apps_rustjnidemo_App_modifyTestStringFromRust
  (JNIEnv *, jclass, jstring);

/*
 * Class:     top_srcres_apps_rustjnidemo_App
 * Method:    actCallFromRust
 * Signature: (Ljava/lang/String;)Ljava/lang/String;
 */
JNIEXPORT jstring JNICALL Java_top_srcres_apps_rustjnidemo_App_actCallFromRust
  (JNIEnv *, jclass, jstring);

/*
 * Class:     top_srcres_apps_rustjnidemo_App
 * Method:    delayInRust
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_top_srcres_apps_rustjnidemo_App_delayInRust
  (JNIEnv *, jclass, jlong);

#ifdef __cplusplus
}
#endif
#endif
