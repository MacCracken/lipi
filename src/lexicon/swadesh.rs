//! Swadesh word lists — core vocabulary for language comparison.
//!
//! The Swadesh list is a set of basic concepts considered resistant to
//! borrowing across languages. This module provides starter lists
//! (25 core items from the Swadesh-100) for the 10 core languages.

use std::borrow::Cow;

use super::{LexEntry, Lexicon, PartOfSpeech};

/// Build a Swadesh starter lexicon for a language.
///
/// Returns `None` for languages without Swadesh data.
#[must_use]
pub fn by_code(code: &str) -> Option<Lexicon> {
    tracing::trace!(code, "swadesh list lookup");
    match code {
        "ar" => Some(arabic()),
        "zh" => Some(mandarin()),
        "hi" => Some(hindi()),
        "ja" => Some(japanese()),
        "es" => Some(spanish()),
        "fr" => Some(french()),
        "de" => Some(german()),
        "ru" => Some(russian()),
        "ko" => Some(korean()),
        "pt" => Some(portuguese()),
        _ => None,
    }
}

/// All language codes with Swadesh data.
#[must_use]
pub fn all_codes() -> &'static [&'static str] {
    &["ar", "zh", "hi", "ja", "es", "fr", "de", "ru", "ko", "pt"]
}

// Helper to reduce boilerplate
fn entry(
    word: &'static str,
    ipa: &'static str,
    gloss: &'static str,
    pos: PartOfSpeech,
    swadesh: u16,
) -> LexEntry {
    LexEntry {
        word: Cow::Borrowed(word),
        ipa: Cow::Borrowed(ipa),
        gloss: Cow::Borrowed(gloss),
        pos,
        frequency_rank: None,
        swadesh_index: Some(swadesh),
    }
}

// Swadesh-25 core indices used across all lists:
// 1=I, 2=you, 3=we, 4=this, 5=that, 6=who, 7=what, 8=not, 9=all, 10=many,
// 11=one, 12=two, 13=big, 14=long, 15=small, 16=woman, 17=man, 18=person,
// 19=fish, 20=bird, 21=dog, 22=tree, 23=water, 24=fire, 25=sun

/// Arabic Swadesh-25 starter list.
#[must_use]
pub fn arabic() -> Lexicon {
    use PartOfSpeech::*;
    Lexicon {
        language_code: Cow::Borrowed("ar"),
        entries: vec![
            entry("أنا", "ʔana", "I", Pronoun, 1),
            entry("أنتَ", "ʔanta", "you", Pronoun, 2),
            entry("نحن", "naħnu", "we", Pronoun, 3),
            entry("هذا", "haːðaː", "this", Determiner, 4),
            entry("ذلك", "ðaːlika", "that", Determiner, 5),
            entry("مَن", "man", "who", Pronoun, 6),
            entry("ما", "maː", "what", Pronoun, 7),
            entry("لا", "laː", "not", Particle, 8),
            entry("كل", "kull", "all", Adjective, 9),
            entry("كثير", "kaθiːr", "many", Adjective, 10),
            entry("واحد", "waːħid", "one", Numeral, 11),
            entry("اثنان", "iθnaːn", "two", Numeral, 12),
            entry("كبير", "kabiːr", "big", Adjective, 13),
            entry("طويل", "tˤawiːl", "long", Adjective, 14),
            entry("صغير", "sˤaɣiːr", "small", Adjective, 15),
            entry("امرأة", "imraʔa", "woman", Noun, 16),
            entry("رجل", "rad͡ʒul", "man", Noun, 17),
            entry("إنسان", "ʔinsaːn", "person", Noun, 18),
            entry("سمك", "samak", "fish", Noun, 19),
            entry("طير", "tˤajr", "bird", Noun, 20),
            entry("كلب", "kalb", "dog", Noun, 21),
            entry("شجرة", "ʃad͡ʒara", "tree", Noun, 22),
            entry("ماء", "maːʔ", "water", Noun, 23),
            entry("نار", "naːr", "fire", Noun, 24),
            entry("شمس", "ʃams", "sun", Noun, 25),
        ],
    }
}

/// Mandarin Chinese Swadesh-25 starter list.
#[must_use]
pub fn mandarin() -> Lexicon {
    use PartOfSpeech::*;
    Lexicon {
        language_code: Cow::Borrowed("zh"),
        entries: vec![
            entry("我", "wɔ˧˩˦", "I", Pronoun, 1),
            entry("你", "ni˧˩˦", "you", Pronoun, 2),
            entry("我们", "wɔ˧˩˦.mən", "we", Pronoun, 3),
            entry("这", "ʈ͡ʂɤ˥˩", "this", Determiner, 4),
            entry("那", "na˥˩", "that", Determiner, 5),
            entry("谁", "ʂeɪ˧˥", "who", Pronoun, 6),
            entry("什么", "ʂən˧˥.mə", "what", Pronoun, 7),
            entry("不", "pu˥˩", "not", Particle, 8),
            entry("都", "tou˥", "all", Adverb, 9),
            entry("多", "two˥", "many", Adjective, 10),
            entry("一", "i˥", "one", Numeral, 11),
            entry("二", "ɤɻ˥˩", "two", Numeral, 12),
            entry("大", "ta˥˩", "big", Adjective, 13),
            entry("长", "ʈ͡ʂʰɑŋ˧˥", "long", Adjective, 14),
            entry("小", "ɕjɑʊ˧˩˦", "small", Adjective, 15),
            entry("女人", "ny˧˩˦.ɻən˧˥", "woman", Noun, 16),
            entry("男人", "nan˧˥.ɻən˧˥", "man", Noun, 17),
            entry("人", "ɻən˧˥", "person", Noun, 18),
            entry("鱼", "y˧˥", "fish", Noun, 19),
            entry("鸟", "njɑʊ˧˩˦", "bird", Noun, 20),
            entry("狗", "koʊ˧˩˦", "dog", Noun, 21),
            entry("树", "ʂu˥˩", "tree", Noun, 22),
            entry("水", "ʂweɪ˧˩˦", "water", Noun, 23),
            entry("火", "xwo˧˩˦", "fire", Noun, 24),
            entry("太阳", "tʰaɪ˥˩.jɑŋ˧˥", "sun", Noun, 25),
        ],
    }
}

/// Hindi Swadesh-25 starter list.
#[must_use]
pub fn hindi() -> Lexicon {
    use PartOfSpeech::*;
    Lexicon {
        language_code: Cow::Borrowed("hi"),
        entries: vec![
            entry("मैं", "mɛ̃ː", "I", Pronoun, 1),
            entry("तू", "tuː", "you", Pronoun, 2),
            entry("हम", "ɦəm", "we", Pronoun, 3),
            entry("यह", "jɛɦ", "this", Determiner, 4),
            entry("वह", "ʋɔɦ", "that", Determiner, 5),
            entry("कौन", "kɔːn", "who", Pronoun, 6),
            entry("क्या", "kjɑː", "what", Pronoun, 7),
            entry("नहीं", "nəɦĩː", "not", Particle, 8),
            entry("सब", "səb", "all", Adjective, 9),
            entry("बहुत", "bəɦʊt", "many", Adjective, 10),
            entry("एक", "eːk", "one", Numeral, 11),
            entry("दो", "doː", "two", Numeral, 12),
            entry("बड़ा", "bəɽɑː", "big", Adjective, 13),
            entry("लंबा", "ləmbɑː", "long", Adjective, 14),
            entry("छोटा", "t͡ʃʰoːʈɑː", "small", Adjective, 15),
            entry("औरत", "ɔːɾət̪", "woman", Noun, 16),
            entry("आदमी", "ɑːd̪miː", "man", Noun, 17),
            entry("इंसान", "ɪnsɑːn", "person", Noun, 18),
            entry("मछली", "mət͡ʃʰliː", "fish", Noun, 19),
            entry("पक्षी", "pəkʂiː", "bird", Noun, 20),
            entry("कुत्ता", "kʊt̪t̪ɑː", "dog", Noun, 21),
            entry("पेड़", "peːɽ", "tree", Noun, 22),
            entry("पानी", "pɑːniː", "water", Noun, 23),
            entry("आग", "ɑːɡ", "fire", Noun, 24),
            entry("सूरज", "suːɾəd͡ʒ", "sun", Noun, 25),
        ],
    }
}

/// Japanese Swadesh-25 starter list.
#[must_use]
pub fn japanese() -> Lexicon {
    use PartOfSpeech::*;
    Lexicon {
        language_code: Cow::Borrowed("ja"),
        entries: vec![
            entry("私", "ʋataɕi", "I", Pronoun, 1),
            entry("あなた", "anata", "you", Pronoun, 2),
            entry("私たち", "ʋataɕitaːtɕi", "we", Pronoun, 3),
            entry("これ", "koɾe", "this", Determiner, 4),
            entry("それ", "soɾe", "that", Determiner, 5),
            entry("誰", "daɾe", "who", Pronoun, 6),
            entry("何", "nani", "what", Pronoun, 7),
            entry("ない", "nai", "not", Particle, 8),
            entry("全部", "zembɯ", "all", Noun, 9),
            entry("多い", "ooi", "many", Adjective, 10),
            entry("一", "itɕi", "one", Numeral, 11),
            entry("二", "ni", "two", Numeral, 12),
            entry("大きい", "oːkiː", "big", Adjective, 13),
            entry("長い", "naɡai", "long", Adjective, 14),
            entry("小さい", "tɕiːsai", "small", Adjective, 15),
            entry("女", "onna", "woman", Noun, 16),
            entry("男", "otoko", "man", Noun, 17),
            entry("人", "çito", "person", Noun, 18),
            entry("魚", "sakana", "fish", Noun, 19),
            entry("鳥", "toɾi", "bird", Noun, 20),
            entry("犬", "inɯ", "dog", Noun, 21),
            entry("木", "ki", "tree", Noun, 22),
            entry("水", "mizɯ", "water", Noun, 23),
            entry("火", "çi", "fire", Noun, 24),
            entry("太陽", "taijoː", "sun", Noun, 25),
        ],
    }
}

/// Spanish Swadesh-25 starter list.
#[must_use]
pub fn spanish() -> Lexicon {
    use PartOfSpeech::*;
    Lexicon {
        language_code: Cow::Borrowed("es"),
        entries: vec![
            entry("yo", "ɟ͡ʝo", "I", Pronoun, 1),
            entry("tú", "tu", "you", Pronoun, 2),
            entry("nosotros", "nosotɾos", "we", Pronoun, 3),
            entry("esto", "esto", "this", Determiner, 4),
            entry("eso", "eso", "that", Determiner, 5),
            entry("quién", "kjen", "who", Pronoun, 6),
            entry("qué", "ke", "what", Pronoun, 7),
            entry("no", "no", "not", Particle, 8),
            entry("todo", "toðo", "all", Adjective, 9),
            entry("mucho", "mutʃo", "many", Adjective, 10),
            entry("uno", "uno", "one", Numeral, 11),
            entry("dos", "dos", "two", Numeral, 12),
            entry("grande", "ɡɾande", "big", Adjective, 13),
            entry("largo", "laɾɣo", "long", Adjective, 14),
            entry("pequeño", "pekeɲo", "small", Adjective, 15),
            entry("mujer", "muxeɾ", "woman", Noun, 16),
            entry("hombre", "ombɾe", "man", Noun, 17),
            entry("persona", "peɾsona", "person", Noun, 18),
            entry("pez", "peθ", "fish", Noun, 19),
            entry("pájaro", "paxaɾo", "bird", Noun, 20),
            entry("perro", "pero", "dog", Noun, 21),
            entry("árbol", "aɾβol", "tree", Noun, 22),
            entry("agua", "aɣwa", "water", Noun, 23),
            entry("fuego", "fweɣo", "fire", Noun, 24),
            entry("sol", "sol", "sun", Noun, 25),
        ],
    }
}

/// French Swadesh-25 starter list.
#[must_use]
pub fn french() -> Lexicon {
    use PartOfSpeech::*;
    Lexicon {
        language_code: Cow::Borrowed("fr"),
        entries: vec![
            entry("je", "ʒə", "I", Pronoun, 1),
            entry("tu", "ty", "you", Pronoun, 2),
            entry("nous", "nu", "we", Pronoun, 3),
            entry("ceci", "səsi", "this", Determiner, 4),
            entry("cela", "səla", "that", Determiner, 5),
            entry("qui", "ki", "who", Pronoun, 6),
            entry("quoi", "kwa", "what", Pronoun, 7),
            entry("ne…pas", "nə pa", "not", Particle, 8),
            entry("tout", "tu", "all", Adjective, 9),
            entry("beaucoup", "boku", "many", Adverb, 10),
            entry("un", "œ̃", "one", Numeral, 11),
            entry("deux", "dø", "two", Numeral, 12),
            entry("grand", "ɡʁɑ̃", "big", Adjective, 13),
            entry("long", "lɔ̃", "long", Adjective, 14),
            entry("petit", "pəti", "small", Adjective, 15),
            entry("femme", "fam", "woman", Noun, 16),
            entry("homme", "ɔm", "man", Noun, 17),
            entry("personne", "pɛʁsɔn", "person", Noun, 18),
            entry("poisson", "pwasɔ̃", "fish", Noun, 19),
            entry("oiseau", "wazo", "bird", Noun, 20),
            entry("chien", "ʃjɛ̃", "dog", Noun, 21),
            entry("arbre", "aʁbʁ", "tree", Noun, 22),
            entry("eau", "o", "water", Noun, 23),
            entry("feu", "fø", "fire", Noun, 24),
            entry("soleil", "sɔlɛj", "sun", Noun, 25),
        ],
    }
}

/// German Swadesh-25 starter list.
#[must_use]
pub fn german() -> Lexicon {
    use PartOfSpeech::*;
    Lexicon {
        language_code: Cow::Borrowed("de"),
        entries: vec![
            entry("ich", "ɪç", "I", Pronoun, 1),
            entry("du", "duː", "you", Pronoun, 2),
            entry("wir", "viːɐ̯", "we", Pronoun, 3),
            entry("dies", "diːs", "this", Determiner, 4),
            entry("jenes", "jeːnəs", "that", Determiner, 5),
            entry("wer", "veːɐ̯", "who", Pronoun, 6),
            entry("was", "vas", "what", Pronoun, 7),
            entry("nicht", "nɪçt", "not", Particle, 8),
            entry("alles", "aləs", "all", Adjective, 9),
            entry("viel", "fiːl", "many", Adjective, 10),
            entry("eins", "aɪ̯ns", "one", Numeral, 11),
            entry("zwei", "t͡svaɪ̯", "two", Numeral, 12),
            entry("groß", "ɡʁoːs", "big", Adjective, 13),
            entry("lang", "laŋ", "long", Adjective, 14),
            entry("klein", "klaɪ̯n", "small", Adjective, 15),
            entry("Frau", "fʁaʊ̯", "woman", Noun, 16),
            entry("Mann", "man", "man", Noun, 17),
            entry("Mensch", "mɛnʃ", "person", Noun, 18),
            entry("Fisch", "fɪʃ", "fish", Noun, 19),
            entry("Vogel", "foːɡl̩", "bird", Noun, 20),
            entry("Hund", "hʊnt", "dog", Noun, 21),
            entry("Baum", "baʊ̯m", "tree", Noun, 22),
            entry("Wasser", "vasɐ", "water", Noun, 23),
            entry("Feuer", "fɔʏ̯ɐ", "fire", Noun, 24),
            entry("Sonne", "zɔnə", "sun", Noun, 25),
        ],
    }
}

/// Russian Swadesh-25 starter list.
#[must_use]
pub fn russian() -> Lexicon {
    use PartOfSpeech::*;
    Lexicon {
        language_code: Cow::Borrowed("ru"),
        entries: vec![
            entry("я", "ja", "I", Pronoun, 1),
            entry("ты", "tɨ", "you", Pronoun, 2),
            entry("мы", "mɨ", "we", Pronoun, 3),
            entry("это", "ˈɛtə", "this", Determiner, 4),
            entry("то", "to", "that", Determiner, 5),
            entry("кто", "kto", "who", Pronoun, 6),
            entry("что", "ʃto", "what", Pronoun, 7),
            entry("не", "nʲe", "not", Particle, 8),
            entry("все", "fsʲe", "all", Adjective, 9),
            entry("много", "ˈmnoɡə", "many", Adjective, 10),
            entry("один", "ɐˈdʲin", "one", Numeral, 11),
            entry("два", "dva", "two", Numeral, 12),
            entry("большой", "bɐlʲˈʂoj", "big", Adjective, 13),
            entry("длинный", "ˈdlʲinːɨj", "long", Adjective, 14),
            entry("маленький", "ˈmalʲɪnʲkʲɪj", "small", Adjective, 15),
            entry("женщина", "ˈʐɛnʲɕːɪnə", "woman", Noun, 16),
            entry("мужчина", "mʊˈɕːinə", "man", Noun, 17),
            entry("человек", "t͡ɕɪlɐˈvʲek", "person", Noun, 18),
            entry("рыба", "ˈrɨbə", "fish", Noun, 19),
            entry("птица", "ˈptʲit͡sə", "bird", Noun, 20),
            entry("собака", "sɐˈbakə", "dog", Noun, 21),
            entry("дерево", "ˈdʲerʲɪvə", "tree", Noun, 22),
            entry("вода", "vɐˈda", "water", Noun, 23),
            entry("огонь", "ɐˈɡonʲ", "fire", Noun, 24),
            entry("солнце", "ˈsont͡sə", "sun", Noun, 25),
        ],
    }
}

/// Korean Swadesh-25 starter list.
#[must_use]
pub fn korean() -> Lexicon {
    use PartOfSpeech::*;
    Lexicon {
        language_code: Cow::Borrowed("ko"),
        entries: vec![
            entry("나", "na", "I", Pronoun, 1),
            entry("너", "nʌ", "you", Pronoun, 2),
            entry("우리", "uɾi", "we", Pronoun, 3),
            entry("이것", "iɡʌt̚", "this", Determiner, 4),
            entry("저것", "t͡ɕʌɡʌt̚", "that", Determiner, 5),
            entry("누구", "nuɡu", "who", Pronoun, 6),
            entry("무엇", "muʌt̚", "what", Pronoun, 7),
            entry("아니", "ani", "not", Particle, 8),
            entry("모두", "modu", "all", Adjective, 9),
            entry("많은", "manɯn", "many", Adjective, 10),
            entry("하나", "hana", "one", Numeral, 11),
            entry("둘", "tul", "two", Numeral, 12),
            entry("큰", "kʰɯn", "big", Adjective, 13),
            entry("긴", "kin", "long", Adjective, 14),
            entry("작은", "t͡ɕaɡɯn", "small", Adjective, 15),
            entry("여자", "jʌd͡ʑa", "woman", Noun, 16),
            entry("남자", "namd͡ʑa", "man", Noun, 17),
            entry("사람", "saɾam", "person", Noun, 18),
            entry("물고기", "mulɡoɡi", "fish", Noun, 19),
            entry("새", "sɛ", "bird", Noun, 20),
            entry("개", "kɛ", "dog", Noun, 21),
            entry("나무", "namu", "tree", Noun, 22),
            entry("물", "mul", "water", Noun, 23),
            entry("불", "pul", "fire", Noun, 24),
            entry("해", "hɛ", "sun", Noun, 25),
        ],
    }
}

/// Portuguese Swadesh-25 starter list.
#[must_use]
pub fn portuguese() -> Lexicon {
    use PartOfSpeech::*;
    Lexicon {
        language_code: Cow::Borrowed("pt"),
        entries: vec![
            entry("eu", "ew", "I", Pronoun, 1),
            entry("tu", "tu", "you", Pronoun, 2),
            entry("nós", "nɔʃ", "we", Pronoun, 3),
            entry("isto", "iʃtu", "this", Determiner, 4),
            entry("isso", "isu", "that", Determiner, 5),
            entry("quem", "kẽj̃", "who", Pronoun, 6),
            entry("que", "kɨ", "what", Pronoun, 7),
            entry("não", "nɐ̃w̃", "not", Particle, 8),
            entry("tudo", "tudu", "all", Adjective, 9),
            entry("muito", "mũjtu", "many", Adjective, 10),
            entry("um", "ũ", "one", Numeral, 11),
            entry("dois", "dojʃ", "two", Numeral, 12),
            entry("grande", "ɡɾɐ̃dɨ", "big", Adjective, 13),
            entry("longo", "lõɡu", "long", Adjective, 14),
            entry("pequeno", "pɨkenu", "small", Adjective, 15),
            entry("mulher", "muʎeɾ", "woman", Noun, 16),
            entry("homem", "ɔmẽj̃", "man", Noun, 17),
            entry("pessoa", "pɨsoɐ", "person", Noun, 18),
            entry("peixe", "pejʃɨ", "fish", Noun, 19),
            entry("pássaro", "pasɐɾu", "bird", Noun, 20),
            entry("cão", "kɐ̃w̃", "dog", Noun, 21),
            entry("árvore", "aɾvuɾɨ", "tree", Noun, 22),
            entry("água", "aɡwɐ", "water", Noun, 23),
            entry("fogo", "foɡu", "fire", Noun, 24),
            entry("sol", "sɔl", "sun", Noun, 25),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_codes_have_lists() {
        for code in all_codes() {
            let lex = by_code(code).unwrap();
            assert_eq!(lex.language_code, *code);
            assert_eq!(lex.entries.len(), 25, "wrong count for {code}");
        }
    }

    #[test]
    fn test_swadesh_indices_sequential() {
        for code in all_codes() {
            let lex = by_code(code).unwrap();
            let sw = lex.swadesh();
            assert_eq!(sw.len(), 25);
            assert_eq!(sw[0].swadesh_index, Some(1));
            assert_eq!(sw[24].swadesh_index, Some(25));
        }
    }

    #[test]
    fn test_cross_language_water() {
        for code in all_codes() {
            let lex = by_code(code).unwrap();
            let water = lex.entries.iter().find(|e| e.gloss == "water");
            assert!(water.is_some(), "missing 'water' in {code}");
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        for code in all_codes() {
            let lex = by_code(code).unwrap();
            let json = serde_json::to_string(&lex).unwrap();
            let back: Lexicon = serde_json::from_str(&json).unwrap();
            assert_eq!(lex, back, "roundtrip failed for {code}");
        }
    }

    #[test]
    fn test_unknown_code() {
        assert!(by_code("xx").is_none());
    }
}
