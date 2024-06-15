pub mod path;
pub mod toast;

use std::path::{Path, PathBuf};

use tauri::{plugin::PluginHandle, Runtime};

use crate::error::Result;
use path::AndroidPath;
use toast::{ToastLength, ToastRequest};

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

            self.0
                .run_mobile_plugin("makeToast", payload)
                .map_err(Into::into)
        }

        #[cfg(not(target_os = "android"))]
        Ok(())
    }

    pub fn private_directory(&self) -> Option<PathBuf> {
        #[cfg(target_os = "android")]
        {
            return self
                .0
                .run_mobile_plugin::<AndroidPath>("getPrivateDirectory", ())
                .ok()
                .map(|path| Path::new(&path.value).to_path_buf());
        }

        #[cfg(not(target_os = "android"))]
        None
    }

    pub fn cache_directory(&self) -> Option<PathBuf> {
        #[cfg(target_os = "android")]
        {
            return self
                .0
                .run_mobile_plugin::<AndroidPath>("getCacheDirectory", ())
                .ok()
                .map(|path| Path::new(&path.value).to_path_buf());
        }

        #[cfg(not(target_os = "android"))]
        None
    }

    pub fn native_lib_directory(&self) -> Option<PathBuf> {
        #[cfg(target_os = "android")]
        {
            return self
                .0
                .run_mobile_plugin::<AndroidPath>("getNativeLibraryDirectory", ())
                .ok()
                .map(|path| Path::new(&path.value).to_path_buf());
        }

        #[cfg(not(target_os = "android"))]
        None
    }
}
