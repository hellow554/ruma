use crate::{parse_id, Error};

pub fn validate(s: &str) -> Result<(), Error> {
    let colon_idx = parse_id(s, &['@'])?;
    let localpart = &s[1..colon_idx];
    let _ = localpart_is_fully_conforming(localpart)?;

    Ok(())
}

/// Check whether the given user id localpart is valid and fully conforming
///
/// Returns an `Err` for invalid user ID localparts, `Ok(false)` for historical user ID localparts
/// and `Ok(true)` for fully conforming user ID localparts.
///
/// With the `compat` feature enabled, this will also return `Ok(false)` for invalid user ID
/// localparts. User IDs that don't even meet the historical user ID restrictions exist in the wild
/// due to Synapse allowing them over federation. This will likely be fixed in an upcoming room
/// version; see [MSC2828](https://github.com/matrix-org/matrix-spec-proposals/pull/2828).
pub const fn localpart_is_fully_conforming(localpart: &str) -> Result<bool, Error> {
    // See https://spec.matrix.org/v1.2/appendices/#user-identifiers
    let is_fully_conforming = {
        // Non-`const` form:
        //
        // localpart
        //     .bytes()
        //     .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'z' | b'-' | b'.' | b'=' | b'_' | b'/'))
        let mut bytes = localpart.as_bytes();
        loop {
            if let Some((b, rest)) = bytes.split_first() {
                if matches!(b, b'0'..=b'9' | b'a'..=b'z' | b'-' | b'.' | b'=' | b'_' | b'/') {
                    bytes = rest;
                } else {
                    break false;
                }
            } else {
                break true;
            }
        }
    };

    // If it's not fully conforming, check if it contains characters that are also disallowed
    // for historical user IDs. If there are, return an error.
    // See https://spec.matrix.org/v1.2/appendices/#historical-user-ids
    #[cfg(not(feature = "compat"))]
    if !is_fully_conforming {
        let has_invalid_chars = {
            // Non-`const` form:
            //
            // localpart.bytes().any(|b| b < 0x21 || b == b':' || b > 0x7E);
            let mut bytes = localpart.as_bytes();
            loop {
                if let Some((&b, rest)) = bytes.split_first() {
                    if b < 0x21 || b == b':' || b > 0x7E {
                        break true;
                    } else {
                        bytes = rest;
                    }
                } else {
                    break false;
                }
            }
        };

        if has_invalid_chars {
            return Err(Error::InvalidCharacters);
        }
    }

    Ok(is_fully_conforming)
}
