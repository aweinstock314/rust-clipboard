use common::ClipboardProvider;
use jni::objects::JString;
use std::{error::Error, ffi::CStr};

pub struct AndroidClipboardContext;

impl ClipboardProvider for AndroidClipboardContext {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(AndroidClipboardContext)
    }

    fn get_contents(&mut self) -> Result<String, Box<dyn Error>> {
        let ctx = ndk_glue::native_activity();

        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let env = vm.attach_current_thread()?;
        let class_ctxt = env.find_class("android/content/Context")?;
        let cb = env.get_static_field(class_ctxt, "CLIPBOARD_SERVICE", "Ljava/lang/String;")?;
        let cb_manager = env
            .call_method(
                ctx.activity(),
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[cb],
            )?
            .l()?;

        let clip_data = env
            .call_method(
                cb_manager,
                "getPrimaryClip",
                "()Landroid/content/ClipData;",
                &[],
            )?
            .l()?;

        let string = env
            .call_method(clip_data, "toString", "()Ljava/lang/String;", &[])?
            .l()?;

        let jstring = JString::from(string.into_inner());

        let ptr = env.get_string_utf_chars(jstring)?;
        let s;
        unsafe {
            s = CStr::from_ptr(ptr).to_owned().into_string()?;
        }
        env.release_string_utf_chars(jstring, ptr)?;
        Ok(s)
    }

    #[allow(deprecated)]
    fn set_contents(&mut self, text: String) -> Result<(), Box<dyn Error>> {
        let ctx = ndk_glue::native_activity();

        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let env = vm.attach_current_thread()?;
        let class_ctxt = env.find_class("android/content/Context")?;
        let cb = env.get_static_field(class_ctxt, "CLIPBOARD_SERVICE", "Ljava/lang/String;")?;
        let cb_manager = env
            .call_method(
                ctx.activity(),
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[cb],
            )?
            .l()?;

        let class_clip_data = env.find_class("android/content/ClipData")?;

        let clip_data = env.call_static_method(
            class_clip_data,
            "newPlainText",
            "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Landroid/content/ClipData;",
            &[
                env.new_string("text").unwrap().into(),
                env.new_string(text).unwrap().into(),
            ],
        )?;

        env.call_method(
            cb_manager,
            "setPrimaryClip",
            "(Landroid/content/ClipData;)V",
            &[clip_data],
        )?
        .v()?;

        Ok(())
    }
}
