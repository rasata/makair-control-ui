// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub fn parse_version_number(version: &str) -> &str {
    // The firmware may sometimes send a version number starting with a "v" or "V" character, \
    //   which we do not want here, hence why we need to filter this out.
    if version.len() > 1 {
        match version.chars().next() {
            Some('v') | Some('V') => {
                // Clear out the 'v' indicator from the version number
                return &version[1..];
            }
            _ => {}
        };
    }

    // Fallback on the identity function (return version as-is)
    version
}

pub fn parse_non_empty_number_to_string(value: usize) -> String {
    if value == 0 {
        "".to_string()
    } else {
        value.to_string()
    }
}

pub fn parse_non_empty_optional_number_to_string(value: Option<usize>) -> String {
    parse_non_empty_number_to_string(value.unwrap_or(0))
}
