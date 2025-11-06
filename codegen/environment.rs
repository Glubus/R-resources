/// Environment/profile preprocessing for resources
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fmt::Write as _;

/// Preprocess XML: remove any element that has a profile attribute not matching the current profile
/// This runs before parsing, so the parser receives only relevant nodes
pub fn preprocess_xml(xml: &str, current_profile: &str) -> String {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut out = String::new();

    // Track whether we are currently skipping a subtree due to mismatched profile
    let mut skip_depth: usize = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                // Check profile on this element
                let mut profile_attr: Option<String> = None;
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"profile" {
                        profile_attr = Some(String::from_utf8_lossy(&attr.value).to_string());
                        break;
                    }
                }
                if skip_depth > 0 {
                    skip_depth += 1;
                } else if let Some(p) = profile_attr {
                    if p != current_profile {
                        skip_depth = 1; // start skipping this subtree
                    }
                }
                if skip_depth == 0 {
                    // write start tag as-is
                    write_start_tag(&mut out, &e);
                }
            }
            Ok(Event::Empty(e)) => {
                // Empty element: skip if mismatched profile
                let mut profile_attr: Option<String> = None;
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"profile" {
                        profile_attr = Some(String::from_utf8_lossy(&attr.value).to_string());
                        break;
                    }
                }
                if skip_depth == 0 {
                    if let Some(p) = profile_attr {
                        if p == current_profile {
                            write_empty_tag(&mut out, &e);
                        } else {
                            // skip this empty element
                        }
                    } else {
                        write_empty_tag(&mut out, &e);
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if skip_depth == 0 {
                    out.push_str(&String::from_utf8_lossy(&e));
                }
            }
            Ok(Event::End(e)) => {
                if skip_depth > 0 {
                    skip_depth -= 1;
                } else {
                    write_end_tag(&mut out, &e);
                }
            }
            Ok(Event::Eof) => break,
            Ok(
                Event::Comment(_)
                | Event::Decl(_)
                | Event::CData(_)
                | Event::PI(_)
                | Event::DocType(_)
                | Event::GeneralRef(_),
            ) => {
                // ignore
            }
            Err(_) => {
                // If preprocessing fails, return original xml to avoid hard failure
                return xml.to_string();
            }
        }
        buf.clear();
    }
    out
}

fn write_start_tag(out: &mut String, e: &quick_xml::events::BytesStart) {
    let name_binding = e.name();
    let name = String::from_utf8_lossy(name_binding.as_ref());
    let _ = write!(out, "<{name}");
    for attr in e.attributes().flatten() {
        let key = String::from_utf8_lossy(attr.key.as_ref());
        let val = String::from_utf8_lossy(&attr.value);
        let _ = write!(out, " {key}=\"{val}\"");
    }
    out.push('>');
}

fn write_empty_tag(out: &mut String, e: &quick_xml::events::BytesStart) {
    let name_binding = e.name();
    let name = String::from_utf8_lossy(name_binding.as_ref());
    let _ = write!(out, "<{name}");
    for attr in e.attributes().flatten() {
        let key = String::from_utf8_lossy(attr.key.as_ref());
        let val = String::from_utf8_lossy(&attr.value);
        let _ = write!(out, " {key}=\"{val}\"");
    }
    out.push_str("/>");
}

fn write_end_tag(out: &mut String, e: &quick_xml::events::BytesEnd) {
    let name_binding = e.name();
    let name = String::from_utf8_lossy(name_binding.as_ref());
    let _ = write!(out, "</{name}>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_build_profile() {
        let profile = get_build_profile();
        assert!(!profile.is_empty());
    }
}
