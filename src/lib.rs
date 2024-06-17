use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

use jni::objects::{JClass, JString};
use jni::sys::{jint, jobject, jvalue};
use jni::JNIEnv;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tokio::runtime::Runtime;

static TOKIO_RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Cannot start Tokio runtime"));

static INSTANCES: Lazy<RwLock<HashMap<i32, Arc<Surreal<Any>>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

static ID_SEQUENCE: Lazy<AtomicI32> = Lazy::new(|| AtomicI32::new(1));

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Surreal_new_1instance<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jobject {
    // Load the Surreal class
    let class = match env.find_class("com/surrealdb/Surreal") {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };
    // Find the constructor
    let constructor = match env.get_method_id(&class, "<init>", "(I)V") {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };
    // Attribute a new ID to each new instance
    let id = ID_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    // Store the instance
    INSTANCES.write().insert(id, Arc::new(Surreal::init()));
    // Build the new instance
    let instance =
        match unsafe { env.new_object_unchecked(&class, constructor, &[jvalue { i: id }]) } {
            Ok(i) => i.into_raw(),
            Err(_) => return std::ptr::null_mut(),
        };
    // Return the instance
    instance
}

#[no_mangle]
pub extern "system" fn Java_com_surrealdb_Surreal_connect<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jint,
    input: JString<'local>,
) {
    // Extract the connection string
    let input: String = match env.get_string(&input) {
        Ok(i) => i.into(),
        Err(_) => {
            let _ = env.throw_new("java/lang/IllegalArgumentException", "Invalid string input");
            return;
        }
    };
    // Retrieve the Surreal instance
    let surreal = match INSTANCES.read().get(&id).cloned() {
        None => {
            let _ = env.throw_new("java/lang/IllegalArgumentException", "Invalid Surreal ID");
            return;
        }
        Some(s) => s,
    };
    // Connect
    if let Err(err) = TOKIO_RUNTIME.block_on(async { surreal.connect(input).await }) {
        let _ = env.throw_new("java/lang/RuntimeException", format!("{err}"));
    }
}
