pub mod pickleball;
pub mod pool;
pub mod table_tennis;
pub mod user;

#[cfg(client)]
use perseus::utils::get_path_prefix_client;

#[cfg(client)]
pub fn get_api_path(path: &str) -> String {
    #[cfg(engine)]
    {
        path.to_string()
    }
    #[cfg(client)]
    {
        let origin = web_sys::window().unwrap().origin();
        let base_path = get_path_prefix_client();
        format!("{}{}{}", origin, base_path, path)
    }
}
