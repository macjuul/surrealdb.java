use std::ptr::null_mut;
use std::sync::Arc;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jboolean, jdouble, jlong, jstring};
use surrealdb::sql::{Number, Value};

use crate::{create_instance, get_value_instance, new_string, release_instance};
use crate::error::SurrealError;

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_deleteInstance<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    release_instance::<Arc<Value>>(id);
    true as jboolean
}


#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_toString<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jstring {
    let value = get_value_instance!(&mut env, id, ||null_mut());
    let s = value.to_string();
    new_string!(&mut env, s, ||null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_toPrettyString<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jstring {
    let value = get_value_instance!(&mut env, id, ||null_mut());
    let s = format!("{value:#}");
    new_string!(&mut env, s, ||null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isArray<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_array() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_getArray<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jlong {
    let value = get_value_instance!(&mut env, id, ||0);
    if value.is_array() {
        create_instance(value)
    } else {
        SurrealError::NullPointerException("Not an array").exception(&mut env, || 0)
    }
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isObject<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_object() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_getObject<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jlong {
    let value = get_value_instance!(&mut env, id, ||0);
    if value.is_object() {
        create_instance(value)
    } else {
        SurrealError::NullPointerException("Not an object").exception(&mut env, || 0)
    }
}


#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isBoolean<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_bool() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_getBoolean<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||0);
    if let Value::Bool(b) = value.as_ref() {
        *b as jboolean
    } else {
        SurrealError::NullPointerException("Not a boolean").exception(&mut env, || 0)
    }
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isBytes<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_bytes() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isLong<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_int() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isDateTime<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_datetime() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isBigDecimal<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_decimal() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isDouble<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_float() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_getDouble<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jdouble {
    let value = get_value_instance!(&mut env, id, ||0.0);
    if let Value::Number(Number::Float(f)) = value.as_ref() {
        *f
    } else {
        SurrealError::NullPointerException("Double").exception(&mut env, || 0.0)
    }
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isGeometry<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_geometry() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isNone<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_none() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isNull<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_null() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isString<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_strand() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_getString<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jstring {
    let value = get_value_instance!(&mut env, id, ||null_mut());
    if let Value::Strand(s) = value.as_ref() {
        new_string!(&mut env, &s.0, ||null_mut())
    } else {
        SurrealError::NullPointerException("Not a string").exception(&mut env, || null_mut())
    }
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isThing<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_thing() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Value_isUuid<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let value = get_value_instance!(&mut env, id, ||false as jboolean);
    value.is_uuid() as jboolean
}