pub mod toast;

use tauri::{plugin::PluginHandle, Runtime};
use toast::{ToastLength, ToastRequest};

use crate::error::Result;

/// Access to the plugin APIs.
pub struct AndroidUtils<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AndroidUtils<R> {
    pub(super) fn init(handle: PluginHandle<R>) -> Self {
        Self(handle)
    }

    pub fn make_toast(&self, message: impl Into<String>, length: ToastLength) -> Result<()> {
        #[cfg(target_os = "android")]
        {
            let long = match length {
                ToastLength::Long => true,
                ToastLength::Short => false,
            };

            let payload = ToastRequest::new(message.into(), long);

            return self
                .0
                .run_mobile_plugin("makeToast", payload)
                .map_err(Into::into);
        }

        #[cfg(not(target_os = "android"))]
        Ok(())
    }
}
