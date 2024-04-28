#[allow(non_snake_case)]
pub mod android {
    use jni::JNIEnv;
    use jni::objects::{JClass, JString};
    use jni::sys::jstring;

    use robonames::generate_short_nickname;

    #[no_mangle]
    pub extern "system" fn Java_com_robosats_RoboIdentities_nativeHello<'local>(mut env: JNIEnv<'local>,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
                                                        _class: JClass<'local>,
                                                        input: JString<'local>)
                                                        -> jstring {
        // First, we have to get the string out of Java. Check jstring `strings`
        // module for more info on how this works.
        let input: String =
            env.get_string(&input).expect("Couldn't get java string!").into();

        // Then we have to create a new Java string to return. Again, more info
        // in the `strings` module.
        let output = env.new_string(format!("Hello, {}!", input))
            .expect("Couldn't create java string!");

        // Finally, extract the raw pointer to return.
        output.into_raw()
    }

    #[no_mangle]
    pub extern "system" fn Java_com_robosats_RoboIdentities_nativeGenerateRoboname<'local>(mut env: JNIEnv<'local>,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
                                                        _class: JClass<'local>,
                                                        initial_string: JString<'local>)
                                                        -> jstring {

        let initial_string: String =
            env.get_string(&initial_string).expect("Couldn't get java string!").into();


        match initial_string.split_once(';') {
            Some((_initial_string, size)) => {
                // Generate Robot Nickname synchronousl. Returns a nickname string.
                let nickname = generate_short_nickname(initial_string);
                match nickname {
                    Ok(nick) => {
                        let output = env.new_string(nickname)
                            .expect("Couldn't create java string!");
                        // Finally, extract the raw pointer to return.
                        output.into_raw()   
                    },
                    Err(_) => todo!(),
                }
            }
            None => todo!(),
        }
    }
}