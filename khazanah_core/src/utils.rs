//! Utility functions.

/// Transliterate a string into a vector of symbols.
pub fn transliterate<F, O>(s: &str, max_pattern_length: usize, map: F) -> Vec<O>
where
    F: Fn(&str) -> Option<O>,
{
    let mut ret = Vec::new();
    let mut i = 0;
    let s_len = s.len();

    while i < s_len {
        let mut j = 0;
        let mut o = Option::<O>::None;
        for k in 1..std::cmp::min(max_pattern_length, s_len - i + 1) {
            let _o = map(s.get(i..i + k).unwrap_or_default());
            if _o.is_some() {
                j = k;
                o = _o;
            }
        }

        if let Some(o) = o {
            ret.push(o);
            i += j;
        } else {
            i += 1;
        }
    }

    ret
}
