package com.plugin.androidUtils

import android.app.Activity
import android.widget.Toast
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
data class ToastArg(
    var message: String? = null,
    var long: Boolean? = null,
)

@TauriPlugin
class AndroidUtils(private val activity: Activity) : Plugin(activity) {
    @Command
    fun makeToast(invoke: Invoke) {
        val args = invoke.parseArgs(ToastArg::class.java)

        if (args.long!!) {
            Toast.makeText(activity, args.message!!, Toast.LENGTH_LONG).show()
        } else {
            Toast.makeText(activity, args.message!!, Toast.LENGTH_SHORT).show()
        }
    }

    @Command
    fun getPrivateDirectory(invoke: Invoke) {
        val privateDir = activity.filesDir.absolutePath
        val ret = JSObject()
        ret.put("path", privateDir)
        invoke.resolve(ret)
    }

    @Command
    fun getCacheDirectory(invoke: Invoke) {
        val cacheDir = activity.cacheDir.absolutePath
        val ret = JSObject()
        ret.put("path", cacheDir)
        invoke.resolve(ret)
    }

    @Command
    fun getNativeLibraryDirectory(invoke: Invoke) {
        val nativeLibraryDir = activity.applicationInfo.nativeLibraryDir
        val ret = JSObject()
        ret.put("path", nativeLibraryDir)
        invoke.resolve(ret)
    }
}
