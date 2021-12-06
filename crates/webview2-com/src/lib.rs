extern crate webview2_com_sys;
pub use webview2_com_sys::*;

#[macro_use]
extern crate webview2_com_macros;

mod callback;
mod options;
mod pwstr;

use std::{fmt, sync::mpsc};

use windows::{
    runtime::HRESULT,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{self, MSG},
    },
};

pub use callback::*;
pub use options::*;
pub use pwstr::*;

#[derive(Debug)]
pub enum Error {
    WindowsError(windows::runtime::Error),
    CallbackError(String),
    TaskCanceled,
    SendError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<windows::runtime::Error> for Error {
    fn from(err: windows::runtime::Error) -> Self {
        Self::WindowsError(err)
    }
}

impl From<HRESULT> for Error {
    fn from(err: HRESULT) -> Self {
        Self::WindowsError(windows::runtime::Error::fast_error(err))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// The WebView2 threading model runs everything on the UI thread, including callbacks which it triggers
/// with `PostMessage`, and we're using this here because it's waiting for some async operations in WebView2
/// to finish before starting the main message loop. As long as there are no pending results in `rx`, it
/// will pump Window messages and check for a result after each message is dispatched.
///
/// `GetMessage` is a blocking call, so if we want to send results from another thread, senders from other
/// threads should "kick" the message loop after sending the result by calling `PostThreadMessage` with an
/// ignorable/unhandled message such as `WM_APP`.
pub fn wait_with_pump<T>(rx: mpsc::Receiver<T>) -> Result<T> {
    let mut msg = MSG::default();
    let hwnd = HWND::default();

    loop {
        if let Ok(result) = rx.try_recv() {
            return Ok(result);
        }

        unsafe {
            match WindowsAndMessaging::GetMessageA(&mut msg, hwnd, 0, 0).0 {
                -1 => {
                    return Err(windows::runtime::Error::from_win32().into());
                }
                0 => return Err(Error::TaskCanceled),
                _ => {
                    WindowsAndMessaging::TranslateMessage(&msg);
                    WindowsAndMessaging::DispatchMessageA(&msg);
                }
            }
        }
    }
}
