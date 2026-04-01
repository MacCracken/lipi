//! Extended phoneme inventories — additional languages beyond the core set.

use super::*;

// ---------------------------------------------------------------------------
// [S] Yucatec Maya
// ---------------------------------------------------------------------------

/// Build the Yucatec Maya phoneme inventory.
///
/// Maya has ejective consonants which are critical for day sign and
/// month name validation in the Mayan calendar (sankhya).
///
/// 21 consonants + 10 vowels (short/long pairs). Stress: fixed (penultimate).
#[must_use]
pub fn yucatec_maya() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("yua", "Yucatec Maya", 31)
        .stress(StressPattern::Fixed)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("k", Plosive, Velar, false)
        .consonant("ʔ", Plosive, Glottal, false)
        .consonant("b", Plosive, Bilabial, true)
        // Ejectives
        .consonant("pʼ", Plosive, Bilabial, false)
        .consonant("tʼ", Plosive, Alveolar, false)
        .consonant("kʼ", Plosive, Velar, false)
        .consonant("t͡sʼ", Affricate, Alveolar, false)
        .consonant("t͡ʃʼ", Affricate, Postalveolar, false)
        // Affricates
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        // Fricatives
        .consonant("s", Fricative, Alveolar, false)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("x", Fricative, Velar, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        // Approximants
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels (5 short + 5 long)
        .vowel("i", Close, Front, false)
        .vowel("iː", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("aː", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("oː", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .vowel("uː", Close, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// African languages
// ---------------------------------------------------------------------------

/// Build the Swahili phoneme inventory.
///
/// 26 consonants + 5 vowels. Stress: penultimate (fixed).
#[must_use]
pub fn swahili() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("sw", "Swahili", 29)
        .stress(StressPattern::Fixed)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("θ", Fricative, Dental, false)
        .consonant("ð", Fricative, Dental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("x", Fricative, Velar, false)
        .consonant("ɣ", Fricative, Velar, true)
        .consonant("h", Fricative, Glottal, false)
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        .consonant("r", Trill, Alveolar, true)
        // Vowels (5-vowel system)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

/// Build the Yoruba phoneme inventory.
///
/// 18 consonants + 7 vowels. Stress: tonal (3-tone system).
#[must_use]
pub fn yoruba() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("yo", "Yoruba", 25)
        .stress(StressPattern::Tonal)
        .tones(vec![
            Cow::Borrowed("˥"), // high
            Cow::Borrowed("˧"), // mid
            Cow::Borrowed("˩"), // low
        ])
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("k͡p", Plosive, LabialVelar, false)
        .consonant("ɡ͡b", Plosive, LabialVelar, true)
        .consonant("f", Fricative, Labiodental, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("h", Fricative, Glottal, false)
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Vowels (7 oral)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

/// Build the Zulu phoneme inventory.
///
/// 42 consonants (including clicks) + 5 vowels. Stress: penultimate.
#[must_use]
pub fn zulu() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("zu", "Zulu", 52)
        .stress(StressPattern::Fixed)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("tʰ", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("kʰ", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Implosives
        .consonant("ɓ", Plosive, Bilabial, true)
        // Affricates
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("t͡sʰ", Affricate, Alveolar, false)
        .consonant("d͡z", Affricate, Alveolar, true)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ɦ", Fricative, Glottal, true)
        .consonant("ɬ", LateralFricative, Alveolar, false)
        .consonant("ɮ", LateralFricative, Alveolar, true)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Approximants
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Clicks (dental, alveolar, lateral)
        .consonant("ǀ", Plosive, Dental, false)
        .consonant("ǀʰ", Plosive, Dental, false)
        .consonant("ɡǀ", Plosive, Dental, true)
        .consonant("ŋǀ", Nasal, Dental, true)
        .consonant("ǃ", Plosive, Alveolar, false)
        .consonant("ǃʰ", Plosive, Alveolar, false)
        .consonant("ɡǃ", Plosive, Alveolar, true)
        .consonant("ŋǃ", Nasal, Alveolar, true)
        .consonant("ǁ", LateralFricative, Alveolar, false)
        .consonant("ǁʰ", LateralFricative, Alveolar, false)
        .consonant("ɡǁ", LateralFricative, Alveolar, true)
        .consonant("ŋǁ", Nasal, Alveolar, true)
        // Vowels
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// Southeast Asian languages
// ---------------------------------------------------------------------------

/// Build the Thai phoneme inventory.
///
/// 21 consonants + 9 vowels. Stress: tonal (5-tone system).
#[must_use]
pub fn thai() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("th", "Thai", 30)
        .stress(StressPattern::Tonal)
        .tones(vec![
            Cow::Borrowed("˧"),   // mid
            Cow::Borrowed("˨˩"),  // low
            Cow::Borrowed("˨˩˦"), // falling
            Cow::Borrowed("˦˥"),  // high
            Cow::Borrowed("˩˧˥"), // rising
        ])
        .consonant("p", Plosive, Bilabial, false)
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("tʰ", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("kʰ", Plosive, Velar, false)
        .consonant("ʔ", Plosive, Glottal, false)
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("t͡ɕʰ", Affricate, Palatal, false)
        .consonant("f", Fricative, Labiodental, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("h", Fricative, Glottal, false)
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ŋ", Nasal, Velar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ɤ", CloseMid, Back, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("u", Close, Back, true)
        .vowel("ɯ", Close, Back, false)
        .build()
}

/// Build the Vietnamese phoneme inventory.
///
/// 22 consonants + 11 vowels. Stress: tonal (6-tone system, Northern).
#[must_use]
pub fn vietnamese() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("vi", "Vietnamese", 33)
        .stress(StressPattern::Tonal)
        .tones(vec![
            Cow::Borrowed("˧"),   // ngang (level)
            Cow::Borrowed("˨˩˦"), // huyền (falling)
            Cow::Borrowed("˧˥"),  // sắc (rising)
            Cow::Borrowed("˧˩˧"), // hỏi (dipping)
            Cow::Borrowed("˧ˀ˥"), // ngã (creaky rising)
            Cow::Borrowed("˨˩ˀ"), // nặng (creaky falling)
        ])
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ʔ", Plosive, Glottal, false)
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("x", Fricative, Velar, false)
        .consonant("ɣ", Fricative, Velar, true)
        .consonant("h", Fricative, Glottal, false)
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("ɯ", Close, Back, false)
        .vowel("ɤ", CloseMid, Back, false)
        .vowel("a", Open, Central, false)
        .vowel("ă", NearOpen, Central, false)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .vowel("ɤ̆", CloseMid, Back, false)
        .build()
}

/// Build the Tagalog phoneme inventory.
///
/// 18 consonants + 5 vowels. Stress: free (contrastive).
#[must_use]
pub fn tagalog() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("tl", "Tagalog", 23)
        .stress(StressPattern::Free)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("ʔ", Plosive, Glottal, false)
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("h", Fricative, Glottal, false)
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// Other language families
// ---------------------------------------------------------------------------

/// Build the Turkish phoneme inventory.
///
/// 20 consonants + 8 vowels. Stress: typically final (agglutinative morphology).
#[must_use]
pub fn turkish() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("tr", "Turkish", 29)
        .stress(StressPattern::Fixed)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ʒ", Fricative, Postalveolar, true)
        .consonant("h", Fricative, Glottal, false)
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels (8-vowel harmony system: front/back × rounded/unrounded × high/low)
        .vowel("i", Close, Front, false)
        .vowel("y", Close, Front, true)
        .vowel("e", CloseMid, Front, false)
        .vowel("ø", CloseMid, Front, true)
        .vowel("ɯ", Close, Back, false)
        .vowel("u", Close, Back, true)
        .vowel("a", Open, Back, false)
        .vowel("o", CloseMid, Back, true)
        .build()
}

/// Build the Finnish phoneme inventory.
///
/// 17 consonants + 8 vowels (short), 8 long. Stress: fixed (initial).
#[must_use]
pub fn finnish() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("fi", "Finnish", 33)
        .stress(StressPattern::Fixed)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("k", Plosive, Velar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("h", Fricative, Glottal, false)
        .consonant("ʋ", Approximant, Labiodental, true)
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ŋ", Nasal, Velar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("f", Fricative, Labiodental, false)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("b", Plosive, Bilabial, true)
        // Vowels (short)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("æ", NearOpen, Front, false)
        .vowel("y", Close, Front, true)
        .vowel("ø", CloseMid, Front, true)
        .vowel("ɑ", Open, Back, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        // Long vowels
        .vowel("iː", Close, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("æː", NearOpen, Front, false)
        .vowel("yː", Close, Front, true)
        .vowel("øː", CloseMid, Front, true)
        .vowel("ɑː", Open, Back, false)
        .vowel("oː", CloseMid, Back, true)
        .vowel("uː", Close, Back, true)
        .build()
}

/// Build the Hawaiian phoneme inventory.
///
/// One of the smallest phoneme inventories in the world.
/// 8 consonants + 5 vowels (short + long). Stress: penultimate.
#[must_use]
pub fn hawaiian() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("haw", "Hawaiian", 18)
        .stress(StressPattern::Fixed)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("k", Plosive, Velar, false)
        .consonant("ʔ", Plosive, Glottal, false)
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("h", Fricative, Glottal, false)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        // Short vowels
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        // Long vowels
        .vowel("iː", Close, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("aː", Open, Central, false)
        .vowel("oː", CloseMid, Back, true)
        .vowel("uː", Close, Back, true)
        .build()
}

/// Build the Nahuatl (Classical) phoneme inventory.
///
/// 16 consonants + 4 vowels (short + long). Stress: penultimate.
#[must_use]
pub fn nahuatl() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("nah", "Nahuatl", 23)
        .stress(StressPattern::Fixed)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("k", Plosive, Velar, false)
        .consonant("kʷ", Plosive, LabialVelar, false)
        .consonant("ʔ", Plosive, Glottal, false)
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("t͡ɬ", Affricate, Alveolar, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ɬ", LateralFricative, Alveolar, false)
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels (short)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        // Long vowels
        .vowel("iː", Close, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("aː", Open, Central, false)
        .vowel("oː", CloseMid, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// Classical / Liturgical languages
// ---------------------------------------------------------------------------

/// Build the Latin (Classical) phoneme inventory.
///
/// 18 consonants + 10 vowels (5 short + 5 long). Stress: penultimate/antepenultimate.
#[must_use]
pub fn latin() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("la", "Latin", 28)
        .stress(StressPattern::Free)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("kʷ", Plosive, LabialVelar, false)
        .consonant("ɡʷ", Plosive, LabialVelar, true)
        .consonant("f", Fricative, Labiodental, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("h", Fricative, Glottal, false)
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ŋ", Nasal, Velar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Short vowels
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        // Long vowels
        .vowel("iː", Close, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("aː", Open, Central, false)
        .vowel("oː", CloseMid, Back, true)
        .vowel("uː", Close, Back, true)
        .build()
}

/// Build the Classical Arabic phoneme inventory.
///
/// 28 consonants + 6 vowels (3 short + 3 long). Stress: rule-based.
#[must_use]
pub fn classical_arabic() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("ar", "Arabic", 34)
        .stress(StressPattern::Free)
        // Plosives
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("tˤ", Plosive, Alveolar, false) // emphatic
        .consonant("dˤ", Plosive, Alveolar, true) // emphatic
        .consonant("k", Plosive, Velar, false)
        .consonant("q", Plosive, Uvular, false)
        .consonant("ʔ", Plosive, Glottal, false)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("θ", Fricative, Dental, false)
        .consonant("ð", Fricative, Dental, true)
        .consonant("ðˤ", Fricative, Dental, true) // emphatic
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("sˤ", Fricative, Alveolar, false) // emphatic
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("x", Fricative, Velar, false)
        .consonant("ɣ", Fricative, Velar, true)
        .consonant("ħ", Fricative, Pharyngeal, false)
        .consonant("ʕ", Fricative, Pharyngeal, true)
        .consonant("h", Fricative, Glottal, false)
        // Affricates
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        // Liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        // Approximants
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels
        .vowel("a", Open, Central, false)
        .vowel("i", Close, Front, false)
        .vowel("u", Close, Back, true)
        .vowel("aː", Open, Central, false)
        .vowel("iː", Close, Front, false)
        .vowel("uː", Close, Back, true)
        .build()
}

/// Build the Koine Greek phoneme inventory.
///
/// The Greek of the New Testament and Hellenistic period.
/// 17 consonants + 5 vowels. Stress: pitch accent (transitioning to stress).
#[must_use]
pub fn koine_greek() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("grc", "Koine Greek", 22)
        .stress(StressPattern::PitchAccent)
        // Plosives (aspirated series lost by Koine period → fricatives)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Fricatives (from earlier aspirated stops)
        .consonant("f", Fricative, Labiodental, false)
        .consonant("θ", Fricative, Dental, false)
        .consonant("x", Fricative, Velar, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        // Liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        // Approximant
        .consonant("j", Approximant, Palatal, true)
        // Vowels (monophthongization underway)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

/// Build the Literary Chinese phoneme inventory.
///
/// Middle Chinese reconstruction (Qieyun system, ~600 CE).
/// 36 initials + 16 vowel nuclei. Stress: tonal (4-tone system).
#[must_use]
pub fn literary_chinese() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("lzh", "Literary Chinese", 40)
        .stress(StressPattern::Tonal)
        .tones(vec![
            Cow::Borrowed("˥"),  // píng (level)
            Cow::Borrowed("˧˥"), // shǎng (rising)
            Cow::Borrowed("˥˩"), // qù (departing)
            Cow::Borrowed("˩ʔ"), // rù (entering/checked)
        ])
        // Plosives (voiceless, aspirated, voiced triads)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("tʰ", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("kʰ", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Affricates
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("t͡sʰ", Affricate, Alveolar, false)
        .consonant("d͡z", Affricate, Alveolar, true)
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("t͡ɕʰ", Affricate, Palatal, false)
        .consonant("d͡ʑ", Affricate, Palatal, true)
        // Fricatives
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ɕ", Fricative, Palatal, false)
        .consonant("ʑ", Fricative, Palatal, true)
        .consonant("x", Fricative, Velar, false)
        .consonant("ɣ", Fricative, Velar, true)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        // Vowels
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ɑ", Open, Back, false)
        .vowel("ɨ", Close, Central, false)
        .vowel("u", Close, Back, true)
        .vowel("o", CloseMid, Back, true)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("ə", Mid, Central, false)
        .vowel("æ", NearOpen, Front, false)
        .vowel("ɐ", NearOpen, Central, false)
        .build()
}

// ---------------------------------------------------------------------------
// Major world languages
// ---------------------------------------------------------------------------

/// Build the Mandarin Chinese (Standard/Putonghua) phoneme inventory.
///
/// Based on the standard Pinyin analysis: 21 initial consonants + 7 vowel nuclei.
/// Tonal: 4 lexical tones + neutral tone.
#[must_use]
pub fn mandarin() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("zh", "Mandarin Chinese", 28)
        .stress(StressPattern::Tonal)
        .tones(vec![
            Cow::Borrowed("˥˥"),  // tone 1: high level
            Cow::Borrowed("˧˥"),  // tone 2: rising
            Cow::Borrowed("˨˩˦"), // tone 3: dipping
            Cow::Borrowed("˥˩"),  // tone 4: falling
            Cow::Borrowed("˧"),   // neutral tone
        ])
        // Bilabials
        .consonant("p", Plosive, Bilabial, false)
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("m", Nasal, Bilabial, true)
        .consonant("f", Fricative, Labiodental, false)
        // Alveolars
        .consonant("t", Plosive, Alveolar, false)
        .consonant("tʰ", Plosive, Alveolar, false)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        // Alveolar sibilants
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("t͡sʰ", Affricate, Alveolar, false)
        .consonant("s", Fricative, Alveolar, false)
        // Retroflex sibilants
        .consonant("ʈ͡ʂ", Affricate, Retroflex, false)
        .consonant("ʈ͡ʂʰ", Affricate, Retroflex, false)
        .consonant("ʂ", Fricative, Retroflex, false)
        .consonant("ɻ", Approximant, Retroflex, true)
        // Palatals (from velar/alveolar before front vowels)
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("t͡ɕʰ", Affricate, Palatal, false)
        .consonant("ɕ", Fricative, Palatal, false)
        // Velars
        .consonant("k", Plosive, Velar, false)
        .consonant("kʰ", Plosive, Velar, false)
        .consonant("x", Fricative, Velar, false)
        // Vowels (nuclear vowels of the syllable finals)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("ɤ", CloseMid, Back, false)
        .vowel("e", OpenMid, Front, false)
        .vowel("i", Close, Front, false)
        .vowel("u", Close, Back, true)
        .vowel("y", Close, Front, true)
        .build()
}

/// Build the Hindi phoneme inventory.
///
/// Standard Hindi (Khariboli). Full aspirated and retroflex series.
/// ~34 consonants + 10 vowels (short/long pairs + schwa). Stress: free.
#[must_use]
pub fn hindi() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("hi", "Hindi", 44)
        .stress(StressPattern::Free)
        // Velars
        .consonant("k", Plosive, Velar, false)
        .consonant("kʰ", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("ɡʱ", Plosive, Velar, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Palatals
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("t͡ɕʰ", Affricate, Palatal, false)
        .consonant("d͡ʑ", Affricate, Palatal, true)
        .consonant("d͡ʑʱ", Affricate, Palatal, true)
        .consonant("ɲ", Nasal, Palatal, true)
        // Retroflexes
        .consonant("ʈ", Plosive, Retroflex, false)
        .consonant("ʈʰ", Plosive, Retroflex, false)
        .consonant("ɖ", Plosive, Retroflex, true)
        .consonant("ɖʱ", Plosive, Retroflex, true)
        .consonant("ɳ", Nasal, Retroflex, true)
        // Dentals
        .consonant("t̪", Plosive, Dental, false)
        .consonant("t̪ʰ", Plosive, Dental, false)
        .consonant("d̪", Plosive, Dental, true)
        .consonant("d̪ʱ", Plosive, Dental, true)
        .consonant("n", Nasal, Alveolar, true)
        // Bilabials
        .consonant("p", Plosive, Bilabial, false)
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("bʱ", Plosive, Bilabial, true)
        .consonant("m", Nasal, Bilabial, true)
        // Approximants & liquids
        .consonant("j", Approximant, Palatal, true)
        .consonant("r", TapFlap, Alveolar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("ʋ", Approximant, Labiodental, true)
        // Fricatives
        .consonant("ɕ", Fricative, Palatal, false)
        .consonant("ʂ", Fricative, Retroflex, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("ɦ", Fricative, Glottal, true)
        // Borrowed / marginal
        .consonant("f", Fricative, Labiodental, false)
        // Vowels (short and long pairs + schwa)
        .vowel("ə", Mid, Central, false) // inherent schwa
        .vowel("i", Close, Front, false)
        .vowel("iː", Close, Front, false)
        .vowel("u", Close, Back, true)
        .vowel("uː", Close, Back, true)
        .vowel("e", CloseMid, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("ɛː", OpenMid, Front, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("ɔː", OpenMid, Back, true)
        .build()
}

/// Build the Japanese phoneme inventory.
///
/// Standard Tokyo Japanese. Mora-timed with pitch accent.
/// ~15 consonants + 5 vowels. Stress: pitch accent.
#[must_use]
pub fn japanese() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("ja", "Japanese", 20)
        .stress(StressPattern::PitchAccent)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Affricates (allophonic before /i/, /u/ — treated as separate in inventory)
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("d͡ʑ", Affricate, Palatal, true)
        // Fricatives
        .consonant("ɸ", Fricative, Bilabial, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ɕ", Fricative, Palatal, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Approximants & liquids
        .consonant("ɾ", TapFlap, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        .consonant("w", Approximant, LabialVelar, true)
        // Vowels (5-vowel system)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, false) // unrounded in standard Japanese
        .build()
}

/// Build the Spanish (Castilian) phoneme inventory.
///
/// Standard peninsular Spanish. 19 consonants + 5 vowels. Stress: free.
#[must_use]
pub fn spanish() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("es", "Spanish", 24)
        .stress(StressPattern::Free)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Dental, false)
        .consonant("d", Plosive, Dental, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Affricates
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("θ", Fricative, Dental, false) // Castilian /c/ and /z/
        .consonant("s", Fricative, Alveolar, false)
        .consonant("x", Fricative, Velar, false)
        .consonant("β", Fricative, Bilabial, true) // allophone of /b/
        .consonant("ð", Fricative, Dental, true) // allophone of /d/
        .consonant("ɣ", Fricative, Velar, true) // allophone of /g/
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        // Liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("ʎ", LateralApproximant, Palatal, true)
        .consonant("r", TapFlap, Alveolar, true)
        .consonant("rr", Trill, Alveolar, true)
        // Approximants
        .consonant("j", Approximant, Palatal, true)
        .consonant("w", Approximant, LabialVelar, true)
        // Vowels
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

/// Build the French phoneme inventory.
///
/// Standard Parisian French. ~20 consonants + 16 vowels (oral + nasal + schwa).
/// Stress: fixed (final syllable).
#[must_use]
pub fn french() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("fr", "French", 36)
        .stress(StressPattern::Fixed)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ʒ", Fricative, Postalveolar, true)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("ʁ", Fricative, Uvular, true)
        // Approximants (semi-vowels)
        .consonant("j", Approximant, Palatal, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("ɥ", Approximant, Palatal, true) // labial-palatal
        // Oral vowels
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("a", Open, Front, false)
        .vowel("ɑ", Open, Back, false)
        .vowel("y", Close, Front, true)
        .vowel("ø", CloseMid, Front, true)
        .vowel("œ", OpenMid, Front, true)
        .vowel("ə", Mid, Central, false)
        .vowel("u", Close, Back, true)
        .vowel("o", CloseMid, Back, true)
        .vowel("ɔ", OpenMid, Back, true)
        // Nasal vowels
        .vowel("ɛ̃", OpenMid, Front, false)
        .vowel("ɑ̃", Open, Back, false)
        .vowel("œ̃", OpenMid, Front, true)
        .vowel("ɔ̃", OpenMid, Back, true)
        .build()
}

/// Build the German (Standard) phoneme inventory.
///
/// Standard High German. ~22 consonants + 15 vowels (short/long pairs + schwa).
/// Stress: free (root-initial tendency).
#[must_use]
pub fn german() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("de", "German", 37)
        .stress(StressPattern::Free)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Affricates
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("p͡f", Affricate, Labiodental, false)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ç", Fricative, Palatal, false) // ich-Laut
        .consonant("x", Fricative, Velar, false) // ach-Laut
        .consonant("h", Fricative, Glottal, false)
        .consonant("ʁ", Fricative, Uvular, true)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Liquids & approximants
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        // Short vowels
        .vowel("ɪ", NearClose, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("ʊ", NearClose, Back, true)
        .vowel("ʏ", NearClose, Front, true)
        .vowel("œ", OpenMid, Front, true)
        .vowel("ə", Mid, Central, false)
        // Long vowels
        .vowel("iː", Close, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("ɛː", OpenMid, Front, false)
        .vowel("aː", Open, Central, false)
        .vowel("oː", CloseMid, Back, true)
        .vowel("uː", Close, Back, true)
        .vowel("yː", Close, Front, true)
        .vowel("øː", CloseMid, Front, true)
        .build()
}

/// Build the Russian phoneme inventory.
///
/// Standard Russian (Moscow norm). Extensive palatalization: hard/soft pairs.
/// ~32 consonants + 6 vowels. Stress: free (contrastive).
#[must_use]
pub fn russian() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("ru", "Russian", 38)
        .stress(StressPattern::Free)
        // Plosives (plain / palatalized pairs)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("pʲ", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("bʲ", Plosive, Bilabial, true)
        .consonant("t", Plosive, Dental, false)
        .consonant("tʲ", Plosive, Dental, false)
        .consonant("d", Plosive, Dental, true)
        .consonant("dʲ", Plosive, Dental, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("kʲ", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("ɡʲ", Plosive, Velar, true)
        // Affricates
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("t͡ɕ", Affricate, Palatal, false)
        // Fricatives (plain / palatalized pairs)
        .consonant("f", Fricative, Labiodental, false)
        .consonant("fʲ", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("vʲ", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("sʲ", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("zʲ", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ʒ", Fricative, Postalveolar, true)
        .consonant("ɕː", Fricative, Palatal, false) // щ
        .consonant("x", Fricative, Velar, false)
        .consonant("xʲ", Fricative, Velar, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("mʲ", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("nʲ", Nasal, Alveolar, true)
        // Liquids & approximants
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("lʲ", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("rʲ", Trill, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels (6-vowel system: stressed realisations)
        .vowel("i", Close, Front, false)
        .vowel("ɨ", Close, Central, false)
        .vowel("u", Close, Back, true)
        .vowel("e", CloseMid, Front, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("a", Open, Central, false)
        .build()
}

/// Build the Korean phoneme inventory.
///
/// Standard Korean (Seoul). Three-way plosive contrast (lax/aspirated/tense).
/// ~19 consonants + 7 vowels. No lexical stress.
#[must_use]
pub fn korean() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("ko", "Korean", 26)
        .stress(StressPattern::Fixed)
        // Plosives: lax / aspirated / tense triads
        .consonant("p", Plosive, Bilabial, false)
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("p͈", Plosive, Bilabial, false) // tense (ㅃ)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("tʰ", Plosive, Alveolar, false)
        .consonant("t͈", Plosive, Alveolar, false) // tense (ㄸ)
        .consonant("k", Plosive, Velar, false)
        .consonant("kʰ", Plosive, Velar, false)
        .consonant("k͈", Plosive, Velar, false) // tense (ㄲ)
        // Affricate: lax / aspirated / tense
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("t͡ɕʰ", Affricate, Palatal, false)
        .consonant("t͡ɕ͈", Affricate, Palatal, false) // tense (ㅉ)
        // Fricatives: lax / tense
        .consonant("s", Fricative, Alveolar, false)
        .consonant("s͈", Fricative, Alveolar, false) // tense (ㅆ)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Liquid
        .consonant("ɾ", TapFlap, Alveolar, true)
        // Vowels (basic monophthongs)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .vowel("ɯ", Close, Back, false)
        .build()
}

/// Build the Portuguese (European) phoneme inventory.
///
/// European Portuguese. Includes oral and nasal vowels.
/// ~19 consonants + 14 vowels (oral + nasal). Stress: free.
#[must_use]
pub fn portuguese() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("pt", "Portuguese", 33)
        .stress(StressPattern::Free)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Dental, false)
        .consonant("d", Plosive, Dental, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Affricates
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ʒ", Fricative, Postalveolar, true)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        // Liquids & approximants
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("ʎ", LateralApproximant, Palatal, true)
        .consonant("r", TapFlap, Alveolar, true) // flap /r/ (intervocalic)
        .consonant("ʁ", Fricative, Uvular, true) // strong /r/ (initial / post-nasal)
        .consonant("j", Approximant, Palatal, true)
        .consonant("w", Approximant, LabialVelar, true)
        // Oral vowels
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ɐ", NearOpen, Central, false)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .vowel("ɨ", Close, Central, false)
        // Nasal vowels
        .vowel("ẽ", CloseMid, Front, false)
        .vowel("ã", Open, Central, false)
        .vowel("õ", CloseMid, Back, true)
        .vowel("ĩ", Close, Front, false)
        .vowel("ũ", Close, Back, true)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! inventory_test {
        ($name:ident, $fn:ident, $code:expr, $consonants:expr, $vowels:expr) => {
            #[test]
            fn $name() {
                let inv = $fn();
                assert_eq!(inv.language_code, $code);
                assert_eq!(inv.consonant_count(), $consonants, "{} consonants", $code);
                assert_eq!(inv.vowel_count(), $vowels, "{} vowels", $code);
                assert_eq!(
                    inv.consonant_count() + inv.vowel_count(),
                    inv.phonemes.len(),
                    "{} total",
                    $code
                );
            }
        };
    }

    inventory_test!(test_maya, yucatec_maya, "yua", 21, 10);
    inventory_test!(test_swahili, swahili, "sw", 26, 5);
    inventory_test!(test_yoruba, yoruba, "yo", 18, 7);
    inventory_test!(test_zulu, zulu, "zu", 42, 5);
    inventory_test!(test_thai, thai, "th", 21, 9);
    inventory_test!(test_vietnamese, vietnamese, "vi", 22, 11);
    inventory_test!(test_tagalog, tagalog, "tl", 18, 5);
    inventory_test!(test_turkish, turkish, "tr", 20, 8);
    inventory_test!(test_finnish, finnish, "fi", 17, 16);
    inventory_test!(test_hawaiian, hawaiian, "haw", 8, 10);
    inventory_test!(test_nahuatl, nahuatl, "nah", 16, 8);
    inventory_test!(test_latin, latin, "la", 18, 10);
    inventory_test!(test_classical_arabic, classical_arabic, "ar", 28, 6);
    inventory_test!(test_koine_greek, koine_greek, "grc", 17, 5);
    inventory_test!(test_literary_chinese, literary_chinese, "lzh", 27, 12);
    inventory_test!(test_mandarin, mandarin, "zh", 21, 7);
    inventory_test!(test_hindi, hindi, "hi", 34, 10);
    inventory_test!(test_japanese, japanese, "ja", 20, 5);
    inventory_test!(test_spanish, spanish, "es", 23, 5);
    inventory_test!(test_french, french, "fr", 21, 16);
    inventory_test!(test_german, german, "de", 23, 16);
    inventory_test!(test_russian, russian, "ru", 36, 6);
    inventory_test!(test_korean, korean, "ko", 19, 7);
    inventory_test!(test_portuguese, portuguese, "pt", 23, 14);

    #[test]
    fn test_maya_ejectives() {
        let m = yucatec_maya();
        assert!(m.has("pʼ"));
        assert!(m.has("tʼ"));
        assert!(m.has("kʼ"));
    }

    #[test]
    fn test_yoruba_tones() {
        let yo = yoruba();
        assert_eq!(yo.tones.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_zulu_clicks() {
        let zu = zulu();
        assert!(zu.has("ǀ"));
        assert!(zu.has("ǃ"));
        assert!(zu.has("ǁ"));
    }

    #[test]
    fn test_hawaiian_minimal() {
        let haw = hawaiian();
        // Hawaiian has one of the smallest inventories
        assert_eq!(haw.consonant_count(), 8);
    }

    #[test]
    fn test_vietnamese_tones() {
        let vi = vietnamese();
        assert_eq!(vi.tones.as_ref().unwrap().len(), 6);
    }

    #[test]
    fn test_arabic_pharyngeals() {
        let ar = classical_arabic();
        assert!(ar.has("ħ"));
        assert!(ar.has("ʕ"));
        assert!(ar.has("q")); // uvular
    }

    #[test]
    fn test_arabic_emphatics() {
        let ar = classical_arabic();
        assert!(ar.has("tˤ"));
        assert!(ar.has("sˤ"));
    }

    #[test]
    fn test_latin_labialized_velars() {
        let la = latin();
        assert!(la.has("kʷ"));
        assert!(la.has("ɡʷ"));
    }

    #[test]
    fn test_koine_greek_stress() {
        let grc = koine_greek();
        assert_eq!(grc.stress, StressPattern::PitchAccent);
    }

    #[test]
    fn test_literary_chinese_tones() {
        let lzh = literary_chinese();
        assert_eq!(lzh.tones.as_ref().unwrap().len(), 4);
    }
}
