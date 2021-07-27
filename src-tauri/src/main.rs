#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{api, CustomMenuItem, Menu, MenuItem, Submenu, WindowBuilder, WindowUrl};

mod cmd;

fn main() {
  fn custom_menu(name: &str) -> CustomMenuItem {
    let c = CustomMenuItem::new(name.to_string(), name);
    return c;
  }
  let menu = Menu::new()
    .add_submenu(Submenu::new(
      // on macOS first menu is always app name
      "Kryp",
      Menu::new()
        .add_native_item(MenuItem::About("Kryp".to_string()))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Services)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::HideOthers)
        .add_native_item(MenuItem::ShowAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit),
    ))
    .add_submenu(Submenu::new(
      "File",
      Menu::new()
        .add_item(custom_menu("New").accelerator("cmdOrControl+N"))
        .add_item(custom_menu("Open...").accelerator("cmdOrControl+O"))
        .add_native_item(MenuItem::Separator)
        .add_item(custom_menu("Save").disabled().accelerator("cmdOrControl+S"))
        .add_item(
          custom_menu("Save As...")
            .disabled()
            .accelerator("shift+cmdOrControl+S"),
        )
        .add_item(custom_menu("Close").accelerator("cmdOrControl+W")),
    ))
    .add_submenu(Submenu::new("Edit", {
      let mut menu = Menu::new();
      menu = menu.add_native_item(MenuItem::Undo);
      menu = menu.add_native_item(MenuItem::Redo);
      menu = menu.add_native_item(MenuItem::Separator);
      menu = menu.add_native_item(MenuItem::Cut);
      menu = menu.add_native_item(MenuItem::Copy);
      menu = menu.add_native_item(MenuItem::Paste);
      #[cfg(not(target_os = "macos"))]
      {
        menu = menu.add_native_item(MenuItem::Separator);
      }
      menu = menu.add_native_item(MenuItem::SelectAll);
      menu
    }))
    .add_submenu(Submenu::new(
      "View",
      Menu::new()
        .add_item(custom_menu("Dashboard").accelerator("cmdOrControl+1"))
        .add_item(custom_menu("Transactions").accelerator("cmdOrControl+2")),
    ))
    .add_submenu(Submenu::new(
      "Window",
      Menu::new()
        .add_native_item(MenuItem::Minimize)
        .add_native_item(MenuItem::Zoom),
    ))
    .add_submenu(Submenu::new(
      "Help",
      Menu::new().add_item(custom_menu("Learn More")),
    ))
    .add_native_item(MenuItem::Copy);

  let ctx = tauri::generate_context!();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![cmd::example])
    .create_window("main", WindowUrl::default(), |win, webview| {
      let win = win
        .title("Mr Tagger")
        .resizable(true)
        .transparent(false)
        .decorations(true)
        .always_on_top(false)
        .inner_size(800.0, 600.0)
        .min_inner_size(300.0, 150.0)
        .skip_taskbar(false)
        .fullscreen(false);
      return (win, webview);
    })
    .menu(menu)
    .on_menu_event(|event| match event.menu_item_id() {
      "learn-more" => {
        api::shell::open(
          "https://github.com/probablykasper/mr-tagger".to_string(),
          None,
        )
        .unwrap();
      }
      _ => {}
    })
    .run(ctx)
    .expect("error while running tauri app");
}
