use std::env;

// 获取是否是开发环境
pub fn get_is_dev() -> bool {
    let is_dev = env::var("IS_DEV").expect("IS_DEV not found");
    if is_dev == "true" {
        return true;
    }
    return false;
}
