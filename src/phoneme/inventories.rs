//! Extended phoneme inventories — additional languages beyond the core set.

use super::*;

// ---------------------------------------------------------------------------
// [S] Yucatec Maya
// ---------------------------------------------------------------------------

/// Build the Yucatec Maya phoneme inventory.
///
/// Maya has ejective consonants and an implosive /ɓ/, which are critical
/// for day sign and month name validation in the Mayan calendar (sankhya).
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
/// 24 consonants + 5 vowels. Stress: penultimate (fixed).
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
/// 47 consonants (including clicks) + 5 vowels. Stress: penultimate.
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
/// 21 consonants + 8 vowels. Stress: typically final (agglutinative morphology).
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
/// 15 consonants + 4 vowels (short + long). Stress: penultimate.
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
}
