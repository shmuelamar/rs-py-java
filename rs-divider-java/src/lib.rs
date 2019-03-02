extern crate jni;
extern crate rsdivider;

// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;
// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping this context
// and getting used after being GC'd.
use jni::objects::JClass;
// This is just a pointer. We'll be returning it from our function.
// We can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jdouble;

// This keeps rust from "mangling" the name and making it unique for this crate.
#[no_mangle]
// This turns off linter warnings because
// the name doesn't conform to conventions.
#[allow(non_snake_case)]
pub extern "system" fn Java_RsDivider_divNumbers(
    _env: JNIEnv,
    // this is the class that owns our static method. Not going to be used,
    // but still needs to have an argument slot
    _class: JClass,
    a: jdouble,
    b: jdouble,
) -> jdouble {
    // convert a and b from long to f64
    let x = a as f64;
    let y = b as f64;

    // convert back the result to long
    let result = rsdivider::div_numbers(x, y);
    result as jdouble
}
