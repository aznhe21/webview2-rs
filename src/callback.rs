use std::sync::mpsc;

use windows::{implement, IUnknown, Interface, HRESULT};

use crate::{
    pwstr::string_from_pwstr,
    webview2::*,
    Microsoft,
    Windows::{self, Win32::Foundation::PWSTR},
};

pub trait ClosureArg {
    type Output: Sized;
}

pub trait InvokeArg<'a> {
    type Input: 'a;

    fn convert(input: Self::Input) -> <Self as ClosureArg>::Output
    where
        Self: ClosureArg;
}

impl ClosureArg for HRESULT {
    type Output = windows::Result<()>;
}

impl<'a> InvokeArg<'a> for HRESULT {
    type Input = Self;

    fn convert(input: HRESULT) -> windows::Result<()> {
        input.ok()
    }
}

impl ClosureArg for PWSTR {
    type Output = String;
}

impl<'a> InvokeArg<'a> for PWSTR {
    type Input = Self;

    fn convert(input: PWSTR) -> String {
        string_from_pwstr(input)
    }
}

impl<I: Interface> ClosureArg for Option<I> {
    type Output = Self;
}

impl<'a, I: 'a + Interface> InvokeArg<'a> for Option<I> {
    type Input = &'a Self;

    fn convert(input: &'a Self) -> <Self as ClosureArg>::Output {
        input.clone()
    }
}

/// Generic closure signature for [`completed_callback`].
pub type CompletedClosure<Arg1, Arg2> = Box<
    dyn FnOnce(<Arg1 as ClosureArg>::Output, <Arg2 as ClosureArg>::Output) -> ::windows::Result<()>,
>;

/// Generic closure signature for [`event_callback`].
pub type EventClosure<Arg1, Arg2> = Box<
    dyn FnMut(<Arg1 as ClosureArg>::Output, <Arg2 as ClosureArg>::Output) -> windows::Result<()>,
>;

#[completed_callback]
pub struct CreateCoreWebView2EnvironmentCompletedHandler(
    ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
    HRESULT,
    Option<ICoreWebView2Environment>,
);

#[completed_callback]
pub struct CreateCoreWebView2ControllerCompletedHandler(
    ICoreWebView2CreateCoreWebView2ControllerCompletedHandler,
    HRESULT,
    Option<ICoreWebView2Controller>,
);

#[event_callback]
pub struct NewBrowserVersionAvailableEventHandler(
    ICoreWebView2NewBrowserVersionAvailableEventHandler,
    Option<ICoreWebView2Environment>,
    Option<IUnknown>,
);

#[completed_callback]
pub struct CreateCoreWebView2CompositionControllerCompletedHandler(
    ICoreWebView2CreateCoreWebView2CompositionControllerCompletedHandler,
    HRESULT,
    Option<ICoreWebView2CompositionController>,
);

#[event_callback]
pub struct ZoomFactorChangedEventHandler(
    ICoreWebView2ZoomFactorChangedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<IUnknown>,
);

#[event_callback]
pub struct MoveFocusRequestedEventHandler(
    ICoreWebView2MoveFocusRequestedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<ICoreWebView2MoveFocusRequestedEventArgs>,
);

#[event_callback]
pub struct FocusChangedEventHandler(
    ICoreWebView2FocusChangedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<IUnknown>,
);

#[event_callback]
pub struct AcceleratorKeyPressedEventHandler(
    ICoreWebView2AcceleratorKeyPressedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<ICoreWebView2AcceleratorKeyPressedEventArgs>,
);

#[event_callback]
pub struct RasterizationScaleChangedEventHandler(
    ICoreWebView2RasterizationScaleChangedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<IUnknown>,
);

#[event_callback]
pub struct NavigationStartingEventHandler(
    ICoreWebView2NavigationStartingEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2NavigationStartingEventArgs>,
);

#[event_callback]
pub struct ContentLoadingEventHandler(
    ICoreWebView2ContentLoadingEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2ContentLoadingEventArgs>,
);

#[event_callback]
pub struct SourceChangedEventHandler(
    ICoreWebView2SourceChangedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2SourceChangedEventArgs>,
);

#[event_callback]
pub struct HistoryChangedEventHandler(
    ICoreWebView2HistoryChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct NavigationCompletedEventHandler(
    ICoreWebView2NavigationCompletedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2NavigationCompletedEventArgs>,
);

#[event_callback]
pub struct ScriptDialogOpeningEventHandler(
    ICoreWebView2ScriptDialogOpeningEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2ScriptDialogOpeningEventArgs>,
);

#[event_callback]
pub struct PermissionRequestedEventHandler(
    ICoreWebView2PermissionRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2PermissionRequestedEventArgs>,
);

#[event_callback]
pub struct ProcessFailedEventHandler(
    ICoreWebView2ProcessFailedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2ProcessFailedEventArgs>,
);

#[completed_callback]
pub struct AddScriptToExecuteOnDocumentCreatedCompletedHandler(
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
    HRESULT,
    PWSTR,
);

#[completed_callback]
pub struct ExecuteScriptCompletedHandler(
    ICoreWebView2ExecuteScriptCompletedHandler,
    HRESULT,
    PWSTR,
);

#[completed_callback]
pub struct CapturePreviewCompletedHandler(ICoreWebView2CapturePreviewCompletedHandler, HRESULT);

#[event_callback]
pub struct WebMessageReceivedEventHandler(
    ICoreWebView2WebMessageReceivedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2WebMessageReceivedEventArgs>,
);

#[completed_callback]
pub struct CallDevToolsProtocolMethodCompletedHandler(
    ICoreWebView2CallDevToolsProtocolMethodCompletedHandler,
    HRESULT,
    PWSTR,
);

#[event_callback]
pub struct NewWindowRequestedEventHandler(
    ICoreWebView2NewWindowRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2NewWindowRequestedEventArgs>,
);

#[event_callback]
pub struct DocumentTitleChangedEventHandler(
    ICoreWebView2DocumentTitleChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct ContainsFullScreenElementChangedEventHandler(
    ICoreWebView2ContainsFullScreenElementChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct WebResourceRequestedEventHandler(
    ICoreWebView2WebResourceRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2WebResourceRequestedEventArgs>,
);

#[event_callback]
pub struct WindowCloseRequestedEventHandler(
    ICoreWebView2WindowCloseRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[completed_callback]
pub struct GetCookiesCompletedHandler(
    ICoreWebView2GetCookiesCompletedHandler,
    HRESULT,
    Option<ICoreWebView2CookieList>,
);
