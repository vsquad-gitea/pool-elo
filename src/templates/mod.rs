pub mod index;
pub mod add_game_form;
pub mod one_v_one_board;
pub mod overall_board;

#[cfg(client)]
use perseus::utils::get_path_prefix_client;

pub fn get_api_path(path: &str) -> String {
    #[cfg(engine)]
    {
        path.to_string()
    }
    #[cfg(client)]
    {
        let path = web_sys::window().unwrap().location().pathname().unwrap();
        // let base_path = get_path_prefix_client();
        // format!("{}{}", base_path, path)
        path.to_string()
    }
}