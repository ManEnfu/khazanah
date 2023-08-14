use std::collections::BTreeMap;

use once_cell::sync::Lazy;

use crate::utils;

/// Transliterate X-SAMPA pronunciation into IPA pronunciation string.
pub fn transliterate_xsampa(s: &str) -> String {
    String::from_iter(utils::transliterate(s, 5, |s| {
        XSAMPA_CHAR_MAP.get(s).copied()
    }))
}

pub static XSAMPA_CHAR_MAP: Lazy<BTreeMap<&'static str, &'static str>> = Lazy::new(xsampa_char_map);

#[doc(hidden)]
fn xsampa_char_map() -> BTreeMap<&'static str, &'static str> {
    let mut map = BTreeMap::new();

    map.insert("a", "a");
    map.insert("b", "b");
    map.insert("b_<", "ɓ");
    map.insert("c", "c");
    map.insert("d", "d");
    map.insert("d`", "ɖ");
    map.insert("d_<", "ɗ");
    map.insert("e", "e");
    map.insert("f", "f");
    map.insert("g", "ɡ");
    map.insert("g_<", "ɠ");
    map.insert("h", "h");
    map.insert("h\\", "ɦ");
    map.insert("i", "i");
    map.insert("j", "j");
    map.insert("j\\", "ʝ");
    map.insert("k", "k");
    map.insert("l", "l");
    map.insert("l`", "ɭ");
    map.insert("l\\", "ɺ");
    map.insert("m", "m");
    map.insert("n", "n");
    map.insert("n`", "ɳ");
    map.insert("o", "o");
    map.insert("p", "p");
    map.insert("p\\", "ɸ");
    map.insert("q", "q");
    map.insert("r", "r");
    map.insert("r`", "ɽ");
    map.insert("r\\", "ɹ");
    map.insert("r\\`", "ɻ");
    map.insert("s", "s");
    map.insert("s`", "ʂ");
    map.insert("s\\", "ɕ");
    map.insert("t", "t");
    map.insert("t`", "ʈ");
    map.insert("u", "u");
    map.insert("v", "v");
    map.insert("v\\", "v");
    map.insert("w", "w");
    map.insert("x", "x");
    map.insert("x\\", "x");
    map.insert("y", "y");
    map.insert("z", "z");
    map.insert("z`", "ʐ");
    map.insert("z\\", "ʑ");

    map.insert("A", "ɑ");
    map.insert("B", "β");
    map.insert("B\\", "ʙ");
    map.insert("C", "ç");
    map.insert("D", "ð");
    map.insert("E", "ɛ");
    map.insert("F", "ɱ");
    map.insert("G", "ɣ");
    map.insert("G\\", "ɢ");
    map.insert("G\\_<", "ʛ");
    map.insert("H", "ɥ");
    map.insert("H\\", "ʜ");
    map.insert("I", "ɪ");
    map.insert("I\\", "ᵻ");
    map.insert("J", "ɲ");
    map.insert("J\\", "ɟ");
    map.insert("J\\_<", "ʄ");
    map.insert("K", "ɬ");
    map.insert("K\\", "ɮ");
    map.insert("L", "ʎ");
    map.insert("L\\", "ʟ");
    map.insert("M", "ɯ");
    map.insert("M\\", "ɰ");
    map.insert("N", "ŋ");
    map.insert("N\\", "ɴ");
    map.insert("O", "ɔ");
    map.insert("O\\", "ʘ");
    map.insert("P", "ʋ");
    map.insert("Q", "ɒ");
    map.insert("R", "ʁ");
    map.insert("R\\", "ʀ");
    map.insert("S", "ʃ");
    map.insert("T", "θ");
    map.insert("U", "ʊ");
    map.insert("U\\", "ᵿ");
    map.insert("V", "ʌ");
    map.insert("W", "ʍ");
    map.insert("X", "ꭓ");
    map.insert("X\\", "ħ");
    map.insert("Y", "ʏ");
    map.insert("Z", "ʒ");

    map.insert(".", ".");
    map.insert("\"", "ˈ");
    map.insert("%", "ˌ");
    map.insert("'", "ʲ");
    map.insert(":", "ː");
    map.insert(":\\", "ˑ");
    map.insert("@", "ə");
    map.insert("@\\", "ɘ");
    map.insert("@`", "ɚ");
    map.insert("{", "æ");
    map.insert("}", "ʉ");
    map.insert("1", "ɨ");
    map.insert("2", "ø");
    map.insert("3", "ɜ");
    map.insert("3\\", "ɞ");
    map.insert("4", "ɾ");
    map.insert("5", "ɫ");
    map.insert("6", "ɐ");
    map.insert("7", "ɤ");
    map.insert("8", "ɵ");
    map.insert("9", "œ");
    map.insert("&", "ɶ");
    map.insert("?", "ʔ");
    map.insert("?\\", "ʕ");
    // map.insert("*", "");
    // map.insert("/", "");
    // map.insert("<", "");
    map.insert("<\\", "ʢ");
    // map.insert(">", "");
    map.insert(">\\", "ʡ");
    map.insert("^", "ꜛ");
    map.insert("!", "ꜜ");
    map.insert("!\\", "ǃ");
    map.insert("|", "|");
    map.insert("|\\", "ǀ");
    map.insert("||", "‖");
    map.insert("|\\|\\", "ǁ");
    map.insert("=\\", "ǂ");
    map.insert("-\\", "\u{0361}");
    map.insert("_", "\u{0361}");
    map.insert("__", "\u{0361}");

    map.insert("_\"", "\u{0308}"); // Centralized
    map.insert("_+", "\u{031f}"); // Advanced
    map.insert("_-", "\u{0320}"); // Retracted
    map.insert("_0", "\u{0325}"); // Voiceless
    map.insert("=", "\u{0329}"); // Syllabic
    map.insert("_=", "\u{329}"); // Syllabic
    map.insert("_>", "ʼ"); // Ejective
    map.insert("_?\\", "ˤ"); // Pharyngealized
    map.insert("_^", "\u{032f}"); // Non syllabic
    map.insert("_}", "\u{031a}"); // No audible release
    map.insert("`", "˞"); // Rhotacization
    map.insert("~", "\u{0303}"); // Nasalization
    map.insert("_~", "\u{0303}"); // Nasalization
    map.insert("_A", "꭪"); // ATR
    map.insert("_a", "\u{033a}"); // Apical
    map.insert("_B", "˩"); // Extra low tone
    map.insert("_c", "\u{031c}"); // Less rounded
    map.insert("_d", "\u{032a}"); // Dental
    map.insert("<F>", "↘"); // Global fall
    map.insert("_G", "ˠ"); // Velarized
    map.insert("_H", "˦"); // High tone
    map.insert("_h", "ʰ"); // Aspirated
    map.insert("_j", "ʲ"); // Palatalized
    map.insert("_k", ""); // Creaky voice
    map.insert("_L", "˨"); // Low tone
    map.insert("_l", "ˡ"); // Lateral release
    map.insert("_M", "˧"); // Mid tone
    map.insert("_m", "\u{033a}"); // Laminal
    map.insert("_N", "\u{033c}"); // Linguolabial
    map.insert("_n", "ⁿ"); // Nasal release
    map.insert("_O", "\u{0339}"); // More rounded
    map.insert("_o", "\u{031e}"); // Lowered
    map.insert("_q", "꭫"); // RTR
    map.insert("<R>", "↗"); // Global rise
    map.insert("_r", "\u{031d}"); // raised
    map.insert("_T", "˥"); // Extra high tone
    map.insert("_t", "\u{0324}"); // Breathy voice
    map.insert("_v", "\u{032c}"); // Voiced
    map.insert("_w", "ʷ"); // Labialized
    map.insert("_X", "\u{0306}"); // Extra short
    map.insert("_x", "\u{033d}"); // Midcentralized

    map.insert("pp\\", "p͡ɸ");
    map.insert("bB", "b͡ꞵ");
    map.insert("pf", "p̪͡f");
    map.insert("bv", "b̪͡v");
    map.insert("tT", "t͡θ");
    map.insert("dD", "d͡ð");
    map.insert("cC", "c͡ç");

    map.insert("ts", "t͡s");
    map.insert("dz", "d͡z");
    map.insert("tS", "t͡ʃ");
    map.insert("dZ", "d͡ʒ");
    map.insert("ts\\", "ʈ͡ʂ");
    map.insert("dz\\", "ɖ͡ʐ");
    map.insert("tK", "t͡ɬ");
    map.insert("kp", "k͡p");
    map.insert("gb", "ɡ͡b");
    map.insert("Nm", "ŋ͡m");

    map.insert(" ", " ");

    map
}
