use std::env;

pub mod index;
pub mod long;

pub fn get_path(path: &str) -> String {
    // Get base path
    match env::var("PERSEUS_BASE_PATH") {
        Ok(env_path) => {
            // Strip the slash on both sides for directory consistency
            // let stripped_env = env_path.trim_start_matches("/").trim_end_matches("/");
            // format!("{stripped_env}/{path}").to_string().trim_end_matches("/").to_owned()
            path.to_owned()
        }
        Err(_) => path.to_owned(),
    }
}
