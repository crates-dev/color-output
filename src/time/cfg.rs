use crate::time::{
    r#type::{from_env_var, Lang},
    time::get_now_time_format,
};

#[test]
fn test_lang() {
    println!("test_lang: {}", from_env_var());
}

#[test]
fn test_now_time() {
    println!("test_now_time: {}", get_now_time_format());
}
