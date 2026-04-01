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
/// 20 consonants + 5 vowels. Stress: pitch accent.
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
/// Standard peninsular Spanish. 23 consonants + 5 vowels. Stress: free.
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
/// Standard Parisian French. 21 consonants + 16 vowels (oral + nasal + schwa).
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
/// Standard High German. 23 consonants + 16 vowels (short/long pairs + schwa).
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
/// 36 consonants + 6 vowels. Stress: free (contrastive).
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
/// 19 consonants + 7 vowels. No lexical stress.
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
/// 23 consonants + 14 vowels (oral + nasal). Stress: free.
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

// ---------------------------------------------------------------------------
// European languages
// ---------------------------------------------------------------------------

/// Build the Italian (Standard) phoneme inventory.
///
/// Standard Italian. Includes four affricates (t͡s/d͡z/t͡ʃ/d͡ʒ) and a full
/// geminate-capable consonant system. 7 oral vowels distinguish mid-high and
/// mid-low series (e/ɛ, o/ɔ) in stressed position.
///
/// 23 consonants + 7 vowels. Stress: free (contrastive).
#[must_use]
pub fn italian() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("it", "Italian", 30)
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
        .consonant("d͡z", Affricate, Alveolar, true)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        // Liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("ʎ", LateralApproximant, Palatal, true)
        .consonant("r", Trill, Alveolar, true)
        // Approximants
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels (7-vowel system)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

/// Build the Dutch (Standard) phoneme inventory.
///
/// Standard Dutch (Algemeen Nederlands). Includes velar fricatives x/ɣ and
/// the uvular trill ʀ. Diphthongs ɛi/œy/ɑu are distinctive.
///
/// 19 consonants + 13 vowels (monophthongs + diphthongs). Stress: free (contrastive).
#[must_use]
pub fn dutch() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("nl", "Dutch", 32)
        .stress(StressPattern::Free)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("x", Fricative, Velar, false)
        .consonant("ɣ", Fricative, Velar, true)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("ʀ", Trill, Uvular, true)
        // Approximants
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Monophthong vowels
        .vowel("i", Close, Front, false)
        .vowel("y", Close, Front, true)
        .vowel("u", Close, Back, true)
        .vowel("e", CloseMid, Front, false)
        .vowel("ø", CloseMid, Front, true)
        .vowel("o", CloseMid, Back, true)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("œ", OpenMid, Front, true)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("a", Open, Central, false)
        // Diphthongs (as vowel units)
        .vowel("ɛi", CloseMid, Front, false)
        .vowel("œy", CloseMid, Front, true)
        .vowel("ɑu", Open, Back, false)
        .build()
}

/// Build the Polish phoneme inventory.
///
/// Standard Polish. Rich sibilant system with three series: alveolar (s/z),
/// retroflex (ʂ/ʐ), and alveolo-palatal (ɕ/ʑ), each with corresponding
/// affricates (t͡s/d͡z, t͡ʂ/d͡ʐ, t͡ɕ/d͡ʑ).
///
/// 29 consonants + 6 vowels. Stress: penultimate (fixed).
#[must_use]
pub fn polish() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("pl", "Polish", 35)
        .stress(StressPattern::Fixed)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Affricates — three series
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("d͡z", Affricate, Alveolar, true)
        .consonant("t͡ʂ", Affricate, Retroflex, false)
        .consonant("d͡ʐ", Affricate, Retroflex, true)
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("d͡ʑ", Affricate, Palatal, true)
        // Fricatives — alveolar series
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        // Fricatives — retroflex series
        .consonant("ʂ", Fricative, Retroflex, false)
        .consonant("ʐ", Fricative, Retroflex, true)
        // Fricatives — alveolo-palatal series
        .consonant("ɕ", Fricative, Palatal, false)
        .consonant("ʑ", Fricative, Palatal, true)
        // Fricatives — other
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("x", Fricative, Velar, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        // Liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        // Approximants
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels (6-vowel system)
        .vowel("i", Close, Front, false)
        .vowel("ɨ", Close, Central, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// South Asian languages
// ---------------------------------------------------------------------------

/// Build the Bengali phoneme inventory.
///
/// Standard Bengali (Shuddo Bangla). Aspirated series, retroflex series,
/// voiced aspirates (breathy voice), and retroflex flap.
///
/// ~30 consonants + 7 vowels. Stress: free.
#[must_use]
pub fn bengali() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("bn", "Bengali", 37)
        .stress(StressPattern::Free)
        // Velars
        .consonant("k", Plosive, Velar, false)
        .consonant("kʰ", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("ɡʱ", Plosive, Velar, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Palatals (affricate series)
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("t͡ɕʰ", Affricate, Palatal, false)
        .consonant("d͡ʑ", Affricate, Palatal, true)
        .consonant("d͡ʑʱ", Affricate, Palatal, true)
        // Retroflexes
        .consonant("ʈ", Plosive, Retroflex, false)
        .consonant("ʈʰ", Plosive, Retroflex, false)
        .consonant("ɖ", Plosive, Retroflex, true)
        .consonant("ɖʱ", Plosive, Retroflex, true)
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
        // Fricatives
        .consonant("s", Fricative, Alveolar, false)
        .consonant("ɦ", Fricative, Glottal, true)
        // Approximants & liquids
        .consonant("j", Approximant, Palatal, true)
        .consonant("r", TapFlap, Alveolar, true)
        .consonant("ɽ", TapFlap, Retroflex, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        // Vowels
        .vowel("i", Close, Front, false)
        .vowel("u", Close, Back, true)
        .vowel("e", CloseMid, Front, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("æ", NearOpen, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ɔ", OpenMid, Back, true)
        .build()
}

/// Build the Tamil phoneme inventory.
///
/// Classical/Standard Tamil. No aspirated consonants; no voiced plosives as
/// phonemes (they are allophones of the voiceless plosives in voiced environments).
/// Rich retroflex series: ʈ ɳ ɭ ɻ.
///
/// ~18 consonants + 10 vowels (5 short + 5 long). Stress: fixed (initial).
#[must_use]
pub fn tamil() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("ta", "Tamil", 28)
        .stress(StressPattern::Fixed)
        // Plosives (voiceless; voiced are allophonic)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("t̪", Plosive, Dental, false)
        .consonant("ʈ", Plosive, Retroflex, false)
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("k", Plosive, Velar, false)
        .consonant("ʔ", Plosive, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɳ", Nasal, Retroflex, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Fricatives
        .consonant("s", Fricative, Alveolar, false)
        .consonant("h", Fricative, Glottal, false)
        // Approximants & liquids
        .consonant("j", Approximant, Palatal, true)
        .consonant("ʋ", Approximant, Labiodental, true)
        .consonant("r", TapFlap, Alveolar, true)
        .consonant("ɻ", Approximant, Retroflex, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("ɭ", LateralApproximant, Retroflex, true)
        // Vowels (5 short + 5 long)
        .vowel("a", Open, Central, false)
        .vowel("aː", Open, Central, false)
        .vowel("i", Close, Front, false)
        .vowel("iː", Close, Front, false)
        .vowel("u", Close, Back, true)
        .vowel("uː", Close, Back, true)
        .vowel("e", CloseMid, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("oː", CloseMid, Back, true)
        .build()
}

/// Build the Urdu phoneme inventory.
///
/// Standard Urdu. Like Hindi but with Persian/Arabic loans: uvular q,
/// fricatives x ɣ, and postalveolar ʃ.
///
/// ~37 consonants + 10 vowels. Stress: free.
#[must_use]
pub fn urdu() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("ur", "Urdu", 47)
        .stress(StressPattern::Free)
        // Velars
        .consonant("k", Plosive, Velar, false)
        .consonant("kʰ", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("ɡʱ", Plosive, Velar, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Uvular (Persian/Arabic)
        .consonant("q", Plosive, Uvular, false)
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
        .consonant("f", Fricative, Labiodental, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false) // Persian/Arabic
        .consonant("x", Fricative, Velar, false) // Persian/Arabic
        .consonant("ɣ", Fricative, Velar, true) // Persian/Arabic
        .consonant("ɦ", Fricative, Glottal, true)
        // Vowels (short and long pairs + schwa)
        .vowel("ə", Mid, Central, false)
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

// ---------------------------------------------------------------------------
// Afro-Asiatic / Chadic languages
// ---------------------------------------------------------------------------

/// Build the Amharic phoneme inventory.
///
/// Ethiopian Semitic (Afro-Asiatic). 27 consonants + 7 vowels = 34 phonemes.
/// Stress: fixed (penultimate). Ejective series pʼ tʼ kʼ t͡ʃʼ t͡sʼ.
/// Gemination is phonemic but modeled as single phonemes here.
#[must_use]
pub fn amharic() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("am", "Amharic", 34)
        .stress(StressPattern::Fixed)
        // Plain plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("ʔ", Plosive, Glottal, false)
        // Ejectives
        .consonant("pʼ", Plosive, Bilabial, false)
        .consonant("tʼ", Plosive, Alveolar, false)
        .consonant("kʼ", Plosive, Velar, false)
        .consonant("t͡ʃʼ", Affricate, Postalveolar, false)
        .consonant("t͡sʼ", Affricate, Alveolar, false)
        // Affricates
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("x", Fricative, Velar, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        // Approximants / liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        .consonant("w", Approximant, LabialVelar, true)
        // Vowels (7-vowel system — the seven Ethiopic orders)
        .vowel("ä", Open, Central, false)
        .vowel("u", Close, Back, true)
        .vowel("i", Close, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɨ", Close, Central, false)
        .vowel("o", CloseMid, Back, true)
        .build()
}

/// Build the Hausa phoneme inventory.
///
/// Chadic (Afro-Asiatic). 25 consonants + 10 vowels (5 short + 5 long) = 35 phonemes.
/// Stress: tonal (2-tone: High / Low). Implosives ɓ ɗ, ejective kʼ,
/// palatalised glottal stop ʔʲ.
#[must_use]
pub fn hausa() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("ha", "Hausa", 35)
        .stress(StressPattern::Tonal)
        .tones(vec![
            Cow::Borrowed("˥"), // high
            Cow::Borrowed("˩"), // low
        ])
        // Plain plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("ʔ", Plosive, Glottal, false)
        .consonant("ʔʲ", Plosive, Palatal, false)
        // Implosives (bilabial + alveolar)
        .consonant("ɓ", Plosive, Bilabial, true)
        .consonant("ɗ", Plosive, Alveolar, true)
        // Ejective
        .consonant("kʼ", Plosive, Velar, false)
        // Affricates
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        .consonant("t͡s", Affricate, Alveolar, false)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        // Approximants / liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        .consonant("w", Approximant, LabialVelar, true)
        // Vowels: 5 short + 5 long
        .vowel("a", Open, Central, false)
        .vowel("aː", Open, Central, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("i", Close, Front, false)
        .vowel("iː", Close, Front, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("oː", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .vowel("uː", Close, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// Austronesian languages
// ---------------------------------------------------------------------------

/// Build the Indonesian phoneme inventory.
///
/// Austronesian (Malay branch). 18 consonants + 6 vowels = 24 phonemes.
/// Stress: fixed (penultimate). Very clean inventory — no clusters,
/// simple phonotactics. Includes palatal nasal ɲ and velar nasal ŋ
/// from the Malay base.
#[must_use]
pub fn indonesian() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("id", "Indonesian", 24)
        .stress(StressPattern::Fixed)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Affricates
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Fricatives
        .consonant("s", Fricative, Alveolar, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Approximant / liquid
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        .consonant("w", Approximant, LabialVelar, true)
        // Vowels (6-vowel system)
        .vowel("i", Close, Front, false)
        .vowel("u", Close, Back, true)
        .vowel("e", CloseMid, Front, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("ə", Mid, Central, false)
        .vowel("a", Open, Central, false)
        .build()
}

// ---------------------------------------------------------------------------
// Middle Eastern / Semitic languages
// ---------------------------------------------------------------------------

/// Build the Modern Iranian Persian (Farsi) phoneme inventory.
///
/// 23 consonants + 6 vowels = 29 phonemes. Stress: free (lexical).
/// Modern Iranian Persian has lost the pharyngeals (ħ, ʕ) and emphatic
/// consonants present in Classical Arabic, but retains the uvular stop q
/// and the velar fricatives x/ɣ. Vowel system: i e æ ɑ o u.
#[must_use]
pub fn persian() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("fa", "Persian", 29)
        .stress(StressPattern::Free)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("q", Plosive, Uvular, false)
        .consonant("ʔ", Plosive, Glottal, false)
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
        .consonant("x", Fricative, Velar, false)
        .consonant("ɣ", Fricative, Velar, true)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        // Approximants / liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        // Vowels (6-vowel system)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("æ", NearOpen, Front, false)
        .vowel("ɑ", Open, Back, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

/// Build the Modern Israeli Hebrew phoneme inventory.
///
/// 23 consonants + 5 vowels = 28 phonemes. Stress: fixed (final/penultimate).
/// Modern Israeli Hebrew retains pharyngeals ħ/ʕ as distinct phonemes in
/// the phonological system (especially in careful/formal speech and among
/// Mizrahi speakers), alongside uvular χ/ʁ. Glottal stop ʔ is marginal.
/// 5-vowel system: i e a o u.
#[must_use]
pub fn hebrew() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("he", "Hebrew", 28)
        .stress(StressPattern::Fixed)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("ʔ", Plosive, Glottal, false)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("χ", Fricative, Uvular, false)
        .consonant("ʁ", Fricative, Uvular, true)
        .consonant("ħ", Fricative, Pharyngeal, false)
        .consonant("ʕ", Fricative, Pharyngeal, true)
        .consonant("h", Fricative, Glottal, false)
        // Affricates
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        // Approximants / liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels (5-vowel system)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

/// Build the Georgian phoneme inventory.
///
/// 28 consonants + 5 vowels = 33 phonemes. Stress: free.
/// Kartvelian language famous for its three-way plosive contrast:
/// voiced / voiceless-aspirated / ejective. Five ejectives: pʼ tʼ kʼ t͡sʼ t͡ʃʼ.
/// Rich consonant clusters (harmonic clusters governed by laryngeal harmony).
#[must_use]
pub fn georgian() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("ka", "Georgian", 33)
        .stress(StressPattern::Free)
        // Plosives — voiced
        .consonant("b", Plosive, Bilabial, true)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("ɡ", Plosive, Velar, true)
        // Plosives — voiceless aspirated
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("tʰ", Plosive, Alveolar, false)
        .consonant("kʰ", Plosive, Velar, false)
        // Plosives — ejective
        .consonant("pʼ", Plosive, Bilabial, false)
        .consonant("tʼ", Plosive, Alveolar, false)
        .consonant("kʼ", Plosive, Velar, false)
        // Glottal stop
        .consonant("ʔ", Plosive, Glottal, false)
        // Affricates — voiced
        .consonant("d͡z", Affricate, Alveolar, true)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Affricates — voiceless aspirated
        .consonant("t͡sʰ", Affricate, Alveolar, false)
        .consonant("t͡ʃʰ", Affricate, Postalveolar, false)
        // Affricates — ejective
        .consonant("t͡sʼ", Affricate, Alveolar, false)
        .consonant("t͡ʃʼ", Affricate, Postalveolar, false)
        // Fricatives
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ʒ", Fricative, Postalveolar, true)
        .consonant("x", Fricative, Velar, false)
        .consonant("ɣ", Fricative, Velar, true)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        // Approximants / liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        // Vowels (5-vowel system)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// Central / Eastern European languages
// ---------------------------------------------------------------------------

/// Build the Czech phoneme inventory.
///
/// 25 consonants + 10 vowels (5 short + 5 long) = 35 phonemes.
/// Stress: fixed (initial syllable). Notable feature: ř (raised alveolar
/// trill, unique to Czech). Full voiced/voiceless plosive and fricative
/// pairs; affricates t͡s t͡ʃ d͡z d͡ʒ.
#[must_use]
pub fn czech() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("cs", "Czech", 35)
        .stress(StressPattern::Fixed)
        // Plosives — voiceless / voiced pairs
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Affricates — voiceless / voiced pairs
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("d͡z", Affricate, Alveolar, true)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Fricatives — voiceless / voiced pairs
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ʒ", Fricative, Postalveolar, true)
        .consonant("x", Fricative, Velar, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        // Liquids — r̝ (ř) is a raised alveolar trill unique to Czech
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("r̝", Trill, Alveolar, true)
        // Approximant
        .consonant("j", Approximant, Palatal, true)
        // Vowels — 5 short + 5 long
        .vowel("a", Open, Central, false)
        .vowel("aː", Open, Central, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("i", Close, Front, false)
        .vowel("iː", Close, Front, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("oː", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .vowel("uː", Close, Back, true)
        .build()
}

/// Build the Hungarian phoneme inventory.
///
/// 25 consonants + 14 vowels (7 short + 7 long) = 39 phonemes.
/// Stress: fixed (initial syllable). Vowel-harmony language with front
/// rounded vowels y/ø and their long counterparts. Palatal stops c/ɟ
/// and palatal nasal ɲ; affricates t͡s d͡z t͡ʃ d͡ʒ.
#[must_use]
pub fn hungarian() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("hu", "Hungarian", 39)
        .stress(StressPattern::Fixed)
        // Plosives — plain and palatal pairs
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("c", Plosive, Palatal, false)
        .consonant("ɟ", Plosive, Palatal, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Affricates — voiceless / voiced pairs
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("d͡z", Affricate, Alveolar, true)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ʒ", Fricative, Postalveolar, true)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        // Liquids / approximants
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels — 7 short + 7 long (front rounded y/ø included)
        .vowel("i", Close, Front, false)
        .vowel("iː", Close, Front, false)
        .vowel("y", Close, Front, true)
        .vowel("yː", Close, Front, true)
        .vowel("e", CloseMid, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("ø", CloseMid, Front, true)
        .vowel("øː", CloseMid, Front, true)
        .vowel("a", Open, Back, true)
        .vowel("aː", Open, Back, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("oː", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .vowel("uː", Close, Back, true)
        .build()
}

/// Build the Romanian phoneme inventory.
///
/// 21 consonants + 7 vowels = 28 phonemes. Stress: free. Romance language
/// with the unique central close vowel ɨ (not found in other Romance
/// languages). Affricates t͡s t͡ʃ d͡ʒ.
#[must_use]
pub fn romanian() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("ro", "Romanian", 28)
        .stress(StressPattern::Free)
        // Plosives — voiceless / voiced pairs
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Affricates
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Fricatives — voiceless / voiced pairs
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ʒ", Fricative, Postalveolar, true)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        // Liquids / approximants
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels — 7 (ɨ is unique among Romance languages)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ə", Mid, Central, false)
        .vowel("ɨ", Close, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// Sino-Tibetan languages
// ---------------------------------------------------------------------------

/// Build the Burmese (Myanmar) phoneme inventory.
///
/// Sino-Tibetan (Tibeto-Burman). Tonal: 4 tones (level, creaky, heavy, stopped)
/// plus register distinctions. Aspirated voiceless series; voiceless nasals
/// m̥ n̥ ŋ̊; glottal stop ʔ.
///
/// 30 consonants + 7 vowels = 37 phonemes. Stress: tonal.
#[must_use]
pub fn burmese() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("my", "Burmese", 37)
        .stress(StressPattern::Tonal)
        .tones(vec![
            Cow::Borrowed("˥"),  // level (flat)
            Cow::Borrowed("˧ˀ"), // creaky (low creaky)
            Cow::Borrowed("˨˩"), // heavy (low falling)
            Cow::Borrowed("˩ʔ"), // stopped/checked (entering)
        ])
        // Bilabial plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        // Alveolar plosives
        .consonant("t", Plosive, Alveolar, false)
        .consonant("tʰ", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        // Velar plosives
        .consonant("k", Plosive, Velar, false)
        .consonant("kʰ", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Glottal stop
        .consonant("ʔ", Plosive, Glottal, false)
        // Affricates
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("t͡ɕʰ", Affricate, Palatal, false)
        .consonant("d͡ʑ", Affricate, Palatal, true)
        // Fricatives
        .consonant("θ", Fricative, Dental, false)
        .consonant("ð", Fricative, Dental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("h", Fricative, Glottal, false)
        // Voiced nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Voiceless nasals (modal voiceless)
        .consonant("m̥", Nasal, Bilabial, false)
        .consonant("n̥", Nasal, Alveolar, false)
        .consonant("ŋ̊", Nasal, Velar, false)
        // Approximants & liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels (7-vowel system)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// Austroasiatic languages
// ---------------------------------------------------------------------------

/// Build the Khmer (Central Khmer) phoneme inventory.
///
/// Austroasiatic (Mon-Khmer). Fixed stress (final syllable). No lexical tones
/// (unusual for mainland Southeast Asia). Rich vowel system: 9 short + 9 long.
/// Implosives ɓ ɗ; aspirated series (pʰ tʰ kʰ); affricates t͡ɕ t͡ɕʰ; glottal stop ʔ.
///
/// 24 consonants + 18 vowels = 42 phonemes. Stress: fixed (final).
#[must_use]
pub fn khmer() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("km", "Khmer", 42)
        .stress(StressPattern::Fixed)
        // Plosives (plain)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("ʔ", Plosive, Glottal, false)
        // Aspirated plosives
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("tʰ", Plosive, Alveolar, false)
        .consonant("kʰ", Plosive, Velar, false)
        // Implosives
        .consonant("ɓ", Plosive, Bilabial, true)
        .consonant("ɗ", Plosive, Alveolar, true)
        // Affricates
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("t͡ɕʰ", Affricate, Palatal, false)
        // Fricatives
        .consonant("s", Fricative, Alveolar, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Approximants & liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", TapFlap, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Short vowels (9)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ɨ", Close, Central, false)
        .vowel("ə", Mid, Central, false)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        // Long vowels (9)
        .vowel("iː", Close, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("ɛː", OpenMid, Front, false)
        .vowel("aː", Open, Central, false)
        .vowel("ɨː", Close, Central, false)
        .vowel("əː", Mid, Central, false)
        .vowel("ɔː", OpenMid, Back, true)
        .vowel("oː", CloseMid, Back, true)
        .vowel("uː", Close, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// Cushitic (Afroasiatic) languages
// ---------------------------------------------------------------------------

/// Build the Somali phoneme inventory.
///
/// Cushitic (Afroasiatic). Tonal: high/low pitch-accent system.
/// Pharyngeals ħ ʕ; uvular q; retroflex ɖ; glottal stop ʔ.
/// 5 short + 5 long vowels.
///
/// 22 consonants + 10 vowels = 32 phonemes. Stress: tonal (high/low accent).
#[must_use]
pub fn somali() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("so", "Somali", 32)
        .stress(StressPattern::Tonal)
        .tones(vec![
            Cow::Borrowed("˥"), // high
            Cow::Borrowed("˩"), // low
        ])
        // Plosives
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("ɖ", Plosive, Retroflex, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("q", Plosive, Uvular, false)
        .consonant("ʔ", Plosive, Glottal, false)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("x", Fricative, Velar, false)
        .consonant("ɣ", Fricative, Velar, true)
        .consonant("ħ", Fricative, Pharyngeal, false)
        .consonant("ʕ", Fricative, Pharyngeal, true)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        // Approximants & liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Short vowels (5)
        .vowel("i", Close, Front, false)
        .vowel("e", OpenMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", OpenMid, Back, true)
        .vowel("u", Close, Back, true)
        // Long vowels (5)
        .vowel("iː", Close, Front, false)
        .vowel("eː", OpenMid, Front, false)
        .vowel("aː", Open, Central, false)
        .vowel("oː", OpenMid, Back, true)
        .vowel("uː", Close, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// Indigenous Americas — Andean and Tupian
// ---------------------------------------------------------------------------

/// Build the Quechua phoneme inventory (Cusco variety, ISO 639-1: qu).
///
/// Quechuan family. Famous for its three-vowel system (i a u) and a rich
/// series of stops contrasting plain, aspirated, and ejective at four places
/// of articulation including uvular q/qʰ/qʼ.
///
/// 24 consonants + 3 vowels = 27 phonemes. Stress: fixed (penultimate).
#[must_use]
pub fn quechua() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("qu", "Quechua", 27)
        .stress(StressPattern::Fixed)
        // Plain stops
        .consonant("p", Plosive, Bilabial, false)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("k", Plosive, Velar, false)
        .consonant("q", Plosive, Uvular, false)
        // Aspirated stops
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("tʰ", Plosive, Alveolar, false)
        .consonant("t͡ʃʰ", Affricate, Postalveolar, false)
        .consonant("kʰ", Plosive, Velar, false)
        .consonant("qʰ", Plosive, Uvular, false)
        // Ejectives
        .consonant("pʼ", Plosive, Bilabial, false)
        .consonant("tʼ", Plosive, Alveolar, false)
        .consonant("t͡ʃʼ", Affricate, Postalveolar, false)
        .consonant("kʼ", Plosive, Velar, false)
        .consonant("qʼ", Plosive, Uvular, false)
        // Fricatives
        .consonant("s", Fricative, Alveolar, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        // Approximants & liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", TapFlap, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Three-vowel system (typologically notable)
        .vowel("i", Close, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("u", Close, Back, true)
        .build()
}

/// Build the Guarani phoneme inventory (ISO 639-1: gn).
///
/// Tupian family. Distinguished by 6 oral + 6 nasal vowels, prenasalized
/// stops ⁿd ⁿɡ (modelled as voiced plosives), and a productive nasal harmony.
/// Glottal stop ʔ is contrastive.
///
/// 16 consonants + 12 vowels = 28 phonemes. Stress: free.
#[must_use]
pub fn guarani() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("gn", "Guarani", 28)
        .stress(StressPattern::Free)
        // Plain stops & glottal
        .consonant("p", Plosive, Bilabial, false)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("k", Plosive, Velar, false)
        .consonant("ʔ", Plosive, Glottal, false)
        // Prenasalized stops (modelled as voiced plosives)
        .consonant("d", Plosive, Alveolar, true) // ⁿd
        .consonant("ɡ", Plosive, Velar, true) // ⁿɡ
        // Affricate
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        // Fricatives
        .consonant("s", Fricative, Alveolar, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Trill & approximants
        .consonant("r", TapFlap, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Oral vowels (6)
        .vowel("a", Open, Central, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("i", Close, Front, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .vowel("ɨ", Close, Central, false)
        // Nasal vowels (6)
        .vowel("ã", Open, Central, false)
        .vowel("ẽ", CloseMid, Front, false)
        .vowel("ĩ", Close, Front, false)
        .vowel("õ", CloseMid, Back, true)
        .vowel("ũ", Close, Back, true)
        .vowel("ɨ̃", Close, Central, false)
        .build()
}

// ---------------------------------------------------------------------------
// North Germanic
// ---------------------------------------------------------------------------

/// Build the Icelandic phoneme inventory (ISO 639-1: is).
///
/// North Germanic. Phonologically distinctive for pre-aspiration (unique in
/// Germanic), voiceless nasals m̥ n̥ ŋ̊, voiceless lateral fricative ɬ,
/// and aspirated stops pʰ tʰ kʰ contrasting with unaspirated p t k.
///
/// 22 consonants + 8 vowels (monophthongs) = 30 phonemes. Stress: free
/// (typically initial in native words).
#[must_use]
pub fn icelandic() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("is", "Icelandic", 30)
        .stress(StressPattern::Free)
        // Aspirated voiceless stops (phonemically distinctive)
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("tʰ", Plosive, Alveolar, false)
        .consonant("kʰ", Plosive, Velar, false)
        // Plain (unaspirated) voiceless stops
        .consonant("p", Plosive, Bilabial, false)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("k", Plosive, Velar, false)
        // Voiced stops
        .consonant("b", Plosive, Bilabial, true)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("ɡ", Plosive, Velar, true)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("θ", Fricative, Dental, false)
        .consonant("ð", Fricative, Dental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("ç", Fricative, Palatal, false)
        .consonant("h", Fricative, Glottal, false)
        // Voiceless lateral fricative (unique feature)
        .consonant("ɬ", LateralFricative, Alveolar, false)
        // Voiceless nasals (pre-aspiration related)
        .consonant("m̥", Nasal, Bilabial, false)
        .consonant("n̥", Nasal, Alveolar, false)
        .consonant("ŋ̊", Nasal, Velar, false)
        // Voiced nasal + trill + approximant
        .consonant("r", Trill, Alveolar, true)
        .consonant("j", Approximant, Palatal, true)
        // Monophthong vowels (8)
        .vowel("i", Close, Front, false)
        .vowel("ɪ", NearClose, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("ɔ", OpenMid, Back, true)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// Atlantic (Niger-Congo) languages
// ---------------------------------------------------------------------------

/// Build the Wolof phoneme inventory.
///
/// Atlantic branch of Niger-Congo, spoken in Senegal and The Gambia.
/// Gemination is phonemic. Prenasalized stops (mb nd ŋɡ ɲɟ) are contrastive.
/// Uvular q and voiceless uvular fricative χ are present.
///
/// 25 consonants + 8 vowels (4 short + 4 long). Stress: free.
#[must_use]
pub fn wolof() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("wo", "Wolof", 33)
        .stress(StressPattern::Free)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("q", Plosive, Uvular, false)
        // Prenasalized stops (phonemic units, not sequences)
        .consonant("mb", Plosive, Bilabial, true)
        .consonant("nd", Plosive, Alveolar, true)
        .consonant("ŋɡ", Plosive, Velar, true)
        .consonant("ɲɟ", Plosive, Palatal, true)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("x", Fricative, Velar, false)
        .consonant("χ", Fricative, Uvular, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ɲ", Nasal, Palatal, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Approximants / liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels — short
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        // Vowels — long
        .vowel("iː", Close, Front, false)
        .vowel("eː", CloseMid, Front, false)
        .vowel("aː", Open, Central, false)
        .vowel("oː", CloseMid, Back, true)
        .build()
}

// ---------------------------------------------------------------------------
// Kra-Dai languages
// ---------------------------------------------------------------------------

/// Build the Lao phoneme inventory.
///
/// Kra-Dai family, closely related to Thai. Tonal (6-tone system).
/// No final consonant clusters. Includes ʔ (glottal stop).
/// Short/long vowel pairs are contrastive; diphthongs included.
///
/// 20 consonants + 9 vowels (short/long pairs + diphthong). Stress: tonal (6 tones).
#[must_use]
pub fn lao() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("lo", "Lao", 29)
        .stress(StressPattern::Tonal)
        .tones(vec![
            Cow::Borrowed("˧"),   // mid (สามัญ)
            Cow::Borrowed("˨˩"),  // low (ເອກ)
            Cow::Borrowed("˥˩"),  // falling (ໂທ)
            Cow::Borrowed("˦˥"),  // high (ຕີ)
            Cow::Borrowed("˩˧˥"), // rising (ຈັດຕະວາ)
            Cow::Borrowed("˧˥"),  // high-rising (ໜ້ໍ)
        ])
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("tʰ", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("kʰ", Plosive, Velar, false)
        .consonant("ʔ", Plosive, Glottal, false)
        // Affricates
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("t͡ɕʰ", Affricate, Palatal, false)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Approximants / liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Vowels — short/long pairs
        .vowel("i", Close, Front, false)
        .vowel("iː", Close, Front, false)
        .vowel("ɯ", Close, Back, false)
        .vowel("ɯː", Close, Back, false)
        .vowel("u", Close, Back, true)
        .vowel("uː", Close, Back, true)
        .vowel("e", CloseMid, Front, false)
        .vowel("eː", CloseMid, Front, false)
        // Diphthong
        .vowel("ia", CloseMid, Front, false)
        .build()
}

// ---------------------------------------------------------------------------
// Mongolic languages
// ---------------------------------------------------------------------------

/// Build the Mongolian phoneme inventory.
///
/// Mongolic family. Vowel harmony (front/back). Includes χ (voiceless uvular
/// fricative) and ɮ (voiced lateral fricative). Short/long vowel pairs are
/// contrastive.
///
/// 21 consonants + 14 vowels (7 short + 7 long). Stress: free.
#[must_use]
pub fn mongolian() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("mn", "Mongolian", 35)
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
        .consonant("d͡z", Affricate, Alveolar, true)
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("x", Fricative, Velar, false)
        .consonant("χ", Fricative, Uvular, false)
        // Lateral fricative
        .consonant("ɮ", LateralFricative, Alveolar, true)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Approximants / liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        // Vowels — back harmony set (short)
        .vowel("a", Open, Back, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .vowel("ɔ", OpenMid, Back, true)
        // Vowels — front harmony set (short)
        .vowel("e", CloseMid, Front, false)
        .vowel("ø", CloseMid, Front, true)
        .vowel("i", Close, Front, false)
        // Vowels — long pairs
        .vowel("aː", Open, Back, false)
        .vowel("oː", CloseMid, Back, true)
        .vowel("uː", Close, Back, true)
        .vowel("ɔː", OpenMid, Back, true)
        .vowel("eː", CloseMid, Front, false)
        .vowel("øː", CloseMid, Front, true)
        .vowel("iː", Close, Front, false)
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
    inventory_test!(test_italian, italian, "it", 23, 7);
    inventory_test!(test_dutch, dutch, "nl", 19, 13);
    inventory_test!(test_polish, polish, "pl", 29, 6);
    inventory_test!(test_amharic, amharic, "am", 27, 7);
    inventory_test!(test_hausa, hausa, "ha", 25, 10);
    inventory_test!(test_indonesian, indonesian, "id", 18, 6);
    inventory_test!(test_czech, czech, "cs", 25, 10);
    inventory_test!(test_hungarian, hungarian, "hu", 25, 14);
    inventory_test!(test_romanian, romanian, "ro", 21, 7);
    inventory_test!(test_burmese, burmese, "my", 30, 7);
    inventory_test!(test_khmer, khmer, "km", 24, 18);
    inventory_test!(test_somali, somali, "so", 22, 10);
    inventory_test!(test_quechua, quechua, "qu", 24, 3);
    inventory_test!(test_guarani, guarani, "gn", 16, 12);
    inventory_test!(test_icelandic, icelandic, "is", 22, 8);

    #[test]
    fn test_czech_unique_r_raised() {
        let cs = czech();
        assert!(
            cs.has("r̝"),
            "Czech ř (raised alveolar trill) must be present"
        );
    }

    #[test]
    fn test_hungarian_front_rounded_vowels() {
        let hu = hungarian();
        assert!(
            hu.has("y"),
            "Hungarian y (front rounded close) must be present"
        );
        assert!(
            hu.has("ø"),
            "Hungarian ø (front rounded close-mid) must be present"
        );
        assert!(hu.has("yː"), "Hungarian long yː must be present");
        assert!(hu.has("øː"), "Hungarian long øː must be present");
    }

    #[test]
    fn test_hungarian_palatal_stops() {
        let hu = hungarian();
        assert!(
            hu.has("c"),
            "Hungarian voiceless palatal stop c must be present"
        );
        assert!(
            hu.has("ɟ"),
            "Hungarian voiced palatal stop ɟ must be present"
        );
    }

    #[test]
    fn test_romanian_central_close_vowel() {
        let ro = romanian();
        assert!(
            ro.has("ɨ"),
            "Romanian ɨ (unique central close vowel) must be present"
        );
    }

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

    #[test]
    fn test_amharic_ejectives() {
        let am = amharic();
        assert!(am.has("pʼ"));
        assert!(am.has("tʼ"));
        assert!(am.has("kʼ"));
        assert!(am.has("t͡ʃʼ"));
        assert!(am.has("t͡sʼ"));
    }

    #[test]
    fn test_hausa_tones() {
        let ha = hausa();
        assert_eq!(ha.tones.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_hausa_implosives() {
        let ha = hausa();
        assert!(ha.has("ɓ"));
        assert!(ha.has("ɗ"));
        assert!(ha.has("kʼ"));
    }

    #[test]
    fn test_indonesian_nasals() {
        let id = indonesian();
        assert!(id.has("ɲ"));
        assert!(id.has("ŋ"));
    }

    #[test]
    fn test_burmese_tones() {
        let my = burmese();
        assert_eq!(my.tones.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn test_burmese_voiceless_nasals() {
        let my = burmese();
        assert!(
            my.has("m̥"),
            "Burmese voiceless bilabial nasal m̥ must be present"
        );
        assert!(
            my.has("n̥"),
            "Burmese voiceless alveolar nasal n̥ must be present"
        );
        assert!(
            my.has("ŋ̊"),
            "Burmese voiceless velar nasal ŋ̊ must be present"
        );
    }

    #[test]
    fn test_burmese_glottal_stop() {
        let my = burmese();
        assert!(my.has("ʔ"), "Burmese glottal stop ʔ must be present");
    }

    #[test]
    fn test_khmer_implosives() {
        let km = khmer();
        assert!(km.has("ɓ"), "Khmer implosive ɓ must be present");
        assert!(km.has("ɗ"), "Khmer implosive ɗ must be present");
    }

    #[test]
    fn test_khmer_vowel_length() {
        let km = khmer();
        // short/long pairs
        assert!(km.has("a"));
        assert!(km.has("aː"));
        assert!(km.has("i"));
        assert!(km.has("iː"));
    }

    #[test]
    fn test_somali_pharyngeals() {
        let so = somali();
        assert!(so.has("ħ"), "Somali pharyngeal ħ must be present");
        assert!(so.has("ʕ"), "Somali pharyngeal ʕ must be present");
    }

    #[test]
    fn test_somali_uvular_retroflex() {
        let so = somali();
        assert!(so.has("q"), "Somali uvular q must be present");
        assert!(so.has("ɖ"), "Somali retroflex ɖ must be present");
    }

    #[test]
    fn test_somali_tones() {
        let so = somali();
        assert_eq!(so.tones.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_quechua_three_vowels() {
        let qu = quechua();
        assert!(qu.has("i"), "Quechua i must be present");
        assert!(qu.has("a"), "Quechua a must be present");
        assert!(qu.has("u"), "Quechua u must be present");
        assert_eq!(qu.vowel_count(), 3, "Quechua three-vowel system");
    }

    #[test]
    fn test_quechua_ejectives_and_aspirates() {
        let qu = quechua();
        assert!(qu.has("pʼ"), "Quechua bilabial ejective pʼ must be present");
        assert!(qu.has("tʼ"), "Quechua alveolar ejective tʼ must be present");
        assert!(qu.has("kʼ"), "Quechua velar ejective kʼ must be present");
        assert!(qu.has("qʼ"), "Quechua uvular ejective qʼ must be present");
        assert!(qu.has("pʰ"), "Quechua aspirated pʰ must be present");
        assert!(qu.has("qʰ"), "Quechua aspirated uvular qʰ must be present");
        assert!(qu.has("q"), "Quechua plain uvular q must be present");
    }

    #[test]
    fn test_quechua_stress_fixed() {
        let qu = quechua();
        assert_eq!(qu.stress, StressPattern::Fixed);
    }

    #[test]
    fn test_guarani_nasal_vowels() {
        let gn = guarani();
        assert!(gn.has("ã"), "Guarani nasal ã must be present");
        assert!(gn.has("ẽ"), "Guarani nasal ẽ must be present");
        assert!(gn.has("ĩ"), "Guarani nasal ĩ must be present");
        assert!(gn.has("õ"), "Guarani nasal õ must be present");
        assert!(gn.has("ũ"), "Guarani nasal ũ must be present");
        assert!(gn.has("ɨ̃"), "Guarani nasal ɨ̃ must be present");
        assert_eq!(gn.vowel_count(), 12, "Guarani 6 oral + 6 nasal vowels");
    }

    #[test]
    fn test_guarani_glottal_stop() {
        let gn = guarani();
        assert!(gn.has("ʔ"), "Guarani glottal stop ʔ must be present");
    }

    #[test]
    fn test_icelandic_voiceless_nasals() {
        let is = icelandic();
        assert!(
            is.has("m̥"),
            "Icelandic voiceless bilabial nasal m̥ must be present"
        );
        assert!(
            is.has("n̥"),
            "Icelandic voiceless alveolar nasal n̥ must be present"
        );
        assert!(
            is.has("ŋ̊"),
            "Icelandic voiceless velar nasal ŋ̊ must be present"
        );
    }

    #[test]
    fn test_icelandic_voiceless_lateral_fricative() {
        let is = icelandic();
        assert!(
            is.has("ɬ"),
            "Icelandic voiceless lateral fricative ɬ must be present"
        );
    }

    #[test]
    fn test_icelandic_aspirated_stops() {
        let is = icelandic();
        assert!(is.has("pʰ"), "Icelandic aspirated pʰ must be present");
        assert!(is.has("tʰ"), "Icelandic aspirated tʰ must be present");
        assert!(is.has("kʰ"), "Icelandic aspirated kʰ must be present");
    }
}
