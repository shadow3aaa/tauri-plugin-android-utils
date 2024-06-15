package com.plugin.androidUtils

import android.app.Activity
import android.widget.Toast
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
data class ToastArg (
  var message: String? = null,
  var long: Boolean? = null,
)

@TauriPlugin
class AndroidUtils(private val activity: Activity): Plugin(activity) {
    @Command
    fun makeToast(invoke: Invoke) {
      val args = invoke.parseArgs(ToastArg::class.java)

      if (args.long!!) {
        Toast.makeText(activity, args.message!!, Toast.LENGTH_LONG).show()
      } else {
        Toast.makeText(activity, args.message!!, Toast.LENGTH_SHORT).show()
      }
    }
}
