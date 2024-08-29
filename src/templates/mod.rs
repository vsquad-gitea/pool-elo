pub mod add_game_form;
pub mod index;
pub mod one_v_one_board;
pub mod overall_board;

#[cfg(client)]
use perseus::utils::get_path_prefix_client;

#[cfg(client)]
pub fn get_api_path(path: &str) -> String {
    let origin = web_sys::window().unwrap().origin();
    let base_path = get_path_prefix_client();
    format!("{}{}{}", origin, base_path, path)
}
