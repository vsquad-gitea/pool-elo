pub mod index;
pub mod add_game_form;
pub mod one_v_one_board;
pub mod overall_board;
pub mod global_state;

#[cfg(client)]
use perseus::utils::get_path_prefix_client;

#[allow(dead_code)]
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
