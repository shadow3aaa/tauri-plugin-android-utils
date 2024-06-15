use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod api;
mod error;

pub use api::{toast, AndroidUtils};
pub use error::{Error, Result};

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the android-utils APIs.
pub trait AndroidUtilsExt<R: Runtime> {
    fn android_utils(&self) -> &AndroidUtils<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AndroidUtilsExt<R> for T {
    fn android_utils(&self) -> &AndroidUtils<R> {
        self.state::<AndroidUtils<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("android-utils")
        .invoke_handler(tauri::generate_handler![])
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            {
                let handle =
                    api.register_android_plugin("com.plugin.androidUtils", "AndroidUtils")?;
                app.manage(AndroidUtils::init(handle));
            }

            Ok(())
        })
        .build()
}
