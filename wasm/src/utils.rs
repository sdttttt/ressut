use fast_xml::{events::attributes::Attributes, Reader};

use std::borrow::Cow;

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.

    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[macro_export]
macro_rules! console_log  {
	($($t:tt)*) => (crate::js_bind::log(&format_args!($($t)*).to_string()))
}

/**
* from attributes get key str;
*/
pub fn attrs_get_str<'a, B: std::io::BufRead>(
    reader: &Reader<B>,
    attrs: Attributes<'a>,
    key: &'a str,
) -> fast_xml::Result<Option<String>> {
    let mut value = None;

    for attribute in attrs {
        let attribute = attribute?;

        if attribute.key != key.as_bytes() {
            break;
        }

        value = Some(
            reader
                .decode(if let Cow::Borrowed(s) = attribute.value {
                    s
                } else {
                    unreachable!();
                })
                .unwrap()
                .to_string(),
        );
    }
    Ok(value)
}


pub fn reader_get_text<B: std::io::BufRead>(
    reader: &mut Reader<B>,
    end: &[u8]
) -> String {
    if let Ok(text) = reader.read_text(end, &mut Vec::new()) {
        text
    } else {
        String::from("")
    }
}