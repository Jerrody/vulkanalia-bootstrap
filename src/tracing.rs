use std::borrow::Cow;
use std::ffi;
use vulkanalia::vk;
use vulkanalia::vk::DebugUtilsMessageSeverityFlagsEXT;

pub unsafe extern "system" fn vulkan_tracing_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    unsafe {
        let callback_data = *p_callback_data;
        let message_id_number = callback_data.message_id_number;

        let message_id_name = if callback_data.message_id_name.is_null() {
            Cow::from("")
        } else {
            ffi::CStr::from_ptr(callback_data.message_id_name).to_string_lossy()
        };

        let message = if callback_data.message.is_null() {
            Cow::from("")
        } else {
            ffi::CStr::from_ptr(callback_data.message).to_string_lossy()
        };

        let message_type = if message_type.contains(vk::DebugUtilsMessageTypeFlagsEXT::GENERAL) {
            "GENERAL"
        } else if message_type.contains(vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE) {
            "PERFORMANCE"
        } else if message_type.contains(vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION) {
            "VALIDATION"
        } else {
            "UNKNOWN"
        };

        match message_severity {
            DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
                tracing::trace!(
                    "[[{message_type}] {message_id_name} ({message_id_number})]: {message}"
                );
            }
            DebugUtilsMessageSeverityFlagsEXT::INFO => {
                tracing::debug!(
                    "[[{message_type}] {message_id_name} ({message_id_number})]: {message}"
                );
            }
            DebugUtilsMessageSeverityFlagsEXT::ERROR => {
                tracing::error!(
                    "[[{message_type}] {message_id_name} ({message_id_number})]: {message}"
                );
            }
            DebugUtilsMessageSeverityFlagsEXT::WARNING => {
                tracing::warn!(
                    "[[{message_type}] {message_id_name} ({message_id_number})]: {message}"
                );
            }
            _ => {
                tracing::debug!(
                    "[[{message_type}] {message_id_name} ({message_id_number})]: {message}"
                );
            }
        }

        vk::FALSE
    }
}
