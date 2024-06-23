use std::sync::Arc;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jboolean, jlong};

use crate::{create_instance, get_sync_value_iterator_instance};
use crate::error::SurrealError;

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_SynchronizedValueIterator_hasNext<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jboolean {
    let iter = get_sync_value_iterator_instance!(&mut env, id, ||false as jboolean);
    let iter = iter.lock();
    (iter.len() > 0) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_SynchronizedValueIterator_next<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jlong,
) -> jlong {
    let iter = get_sync_value_iterator_instance!(&mut env, id, ||0);
    let mut iter = iter.lock();
    if let Some(v) = iter.next() {
        create_instance(Arc::new(v))
    } else {
        SurrealError::NoSuchElementException.exception(&mut env, || 0)
    }
}