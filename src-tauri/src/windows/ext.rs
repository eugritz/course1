#[cfg(unix)]
use gtk::prelude::*;
use tauri::Manager;
#[cfg(windows)]
use windows::Win32::UI::{
    Input::KeyboardAndMouse::EnableWindow,
    WindowsAndMessaging::{GetDesktopWindow, GetWindow, GW_OWNER},
};

pub trait SetAncestor<R: tauri::Runtime> {
    fn ancestor(self, window: &tauri::Window<R>) -> Self;
}

#[cfg(windows)]
impl<R: tauri::Runtime> SetAncestor<R> for tauri::WindowBuilder<'_> {
    fn ancestor(self, window: &tauri::Window<R>) -> Self {
        self.owner_window(window.hwnd().unwrap())
    }
}

#[cfg(unix)]
impl<R: tauri::Runtime> SetAncestor<R> for tauri::WindowBuilder<'_> {
    fn ancestor(self, window: &tauri::Window<R>) -> Self {
        self
    }
}

pub trait SetModal {
    fn set_modal(&self, is_modal: bool);
}

#[cfg(windows)]
impl<R: tauri::Runtime> SetModal for tauri::Window<R> {
    fn set_modal(&self, is_modal: bool) {
        if is_modal {
            let hwnd = self.hwnd().unwrap();
            unsafe {
                let parent = GetWindow(hwnd, GW_OWNER);
                if parent == GetDesktopWindow() {
                    return;
                }

                EnableWindow(parent, false);
            }

        } else {
            let hwnd = self.hwnd().unwrap();
            unsafe {
                let parent = GetWindow(hwnd, GW_OWNER);
                EnableWindow(parent, true);
            }
        }
    }
}

#[cfg(unix)]
impl<R: tauri::Runtime> SetModal for tauri::Window<R> {
    fn set_modal(&self, is_modal: bool) {
        if is_modal {
            let gtk_window = self.gtk_window().unwrap();
            if let Some(parent_window) = self.get_focused_window() {
                let gtk_parent = parent_window.gtk_window().unwrap();
                gtk_window.set_modal(true);
                gtk_window.set_transient_for(Some(&gtk_parent));
            }
        } else {
            let gtk_window = self.gtk_window().unwrap();
            gtk_window.set_modal(false);
        }
    }
}

#[cfg(windows)]
pub fn get_parent_window(window: &tauri::Window) -> Option<tauri::Window> {
    let hwnd_parent = unsafe { GetWindow(window.hwnd().unwrap(), GW_OWNER) };

    for (_, window) in window.app_handle().windows() {
        let hwnd_window = window.hwnd().unwrap();
        if hwnd_parent == hwnd_window {
            return Some(window);
        }
    }

    None
}

#[cfg(unix)]
pub fn get_parent_window(window: &tauri::Window) -> Option<tauri::Window> {
    let gtk_parent = window
        .gtk_window()
        .unwrap()
        .transient_for()
        .unwrap();

    for (_, window) in window.app_handle().windows() {
        let gtk_window = window.gtk_window().unwrap();
        if gtk_window == gtk_parent {
            return Some(window);
        }
    }

    None
}
