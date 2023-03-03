use std::collections::BTreeSet;

use super::char::IPAChar;
use lazy_static::lazy_static;

lazy_static! {
    /// Category of consonants.
    pub static ref CONSONANTS: BTreeSet<IPAChar> = [
        IPAChar::VoicelessBilabialPlosive,
        IPAChar::VoicedBilabialPlosive,
        IPAChar::VoicelessAlveolarPlosive,
        IPAChar::VoicedAlveolarPlosive,
        IPAChar::VoicelessRetroflexPlosive,
        IPAChar::VoicedRetroflexPlosive,
        IPAChar::VoicelessPalatalPlosive,
        IPAChar::VoicedPalatalPlosive,
        IPAChar::VoicelessVelarPlosive,
        IPAChar::VoicedVelarPlosive,
        IPAChar::VoicelessUvularPlosive,
        IPAChar::VoicedUvularPlosive,
        IPAChar::PharyngealPlosive,
        IPAChar::GlottalPlosive,

        IPAChar::VoicedBilabialNasal,
        IPAChar::VoicedLabiodentalNasal,
        IPAChar::VoicedAlveolarNasal,
        IPAChar::VoicedRetroflexNasal,
        IPAChar::VoicedPalatalNasal,
        IPAChar::VoicedVelarNasal,
        IPAChar::VoicedUvularNasal,

        IPAChar::VoicedBilabialTrill,
        IPAChar::VoicedAlveolarTrill,
        IPAChar::VoicedUvularTrill,
        IPAChar::VoicelessPharyngealTrill,
        IPAChar::VoicedPharyngealTrill,

        IPAChar::VoicedLabiodentalFlap,
        IPAChar::VoicedAlveolarFlap,
        IPAChar::VoicedRetroflexFlap,

        IPAChar::VoicelessBilabialFricative,
        IPAChar::VoicedBilabialFricative,
        IPAChar::VoicelessLabiodentalFricative,
        IPAChar::VoicedLabiodentalFricative,
        IPAChar::VoicelessDentalFricative,
        IPAChar::VoicedDentalFricative,
        IPAChar::VoicelessAlveolarFricative,
        IPAChar::VoicedAlveolarFricative,
        IPAChar::VoicelessPostalveolarFricative,
        IPAChar::VoicedPostalveolarFricative,
        IPAChar::VoicelessRetroflexFricative,
        IPAChar::VoicedRetroflexFricative,
        IPAChar::VoicelessPalatalFricative,
        IPAChar::VoicedPalatalFricative,
        IPAChar::VoicelessVelarFricative,
        IPAChar::VoicedVelarFricative,
        IPAChar::VoicelessUvularFricative,
        IPAChar::VoicedUvularFricative,
        IPAChar::VoicelessPharyngealFricative,
        IPAChar::VoicedPharygealFricative,
        IPAChar::VoicelessGlottalFricative,
        IPAChar::VoicedGlottalFricative,

        IPAChar::VoicelessAlveolarLateralFricative,
        IPAChar::VoicedAlveolarLateralFricative,

        IPAChar::VoicedLabiodentalApproximant,
        IPAChar::VoicedAlveolarApproximant,
        IPAChar::VoicedRetroflexApproximant,
        IPAChar::VoicedPalatalApproximant,
        IPAChar::VoicedVelarApproximant,

        IPAChar::VoicedAlveolarLateralApproximant,
        IPAChar::VoicedRetroflexLateralApproximant,
        IPAChar::VoicedPalatalLateralApproximant,
        IPAChar::VoicedVelarLateralApproximant,

        IPAChar::VoicelessBilabialImplosive,
        IPAChar::VoicedBilabialImplosive,
        IPAChar::VoicelessAlveolarImplosive,
        IPAChar::VoicedAlveolarImplosive,
        IPAChar::VoicelessPalatalImplosive,
        IPAChar::VoicedPalatalImplosive,
        IPAChar::VoicelessVelarImplosive,
        IPAChar::VoicedVelarImplosive,
        IPAChar::VoicelessUvularImplosive,
        IPAChar::VoicedUvularImplosive,

        IPAChar::BilabialClick,
        IPAChar::DentalClick,
        IPAChar::AlveolarClick,
        IPAChar::PalatalClick,
        IPAChar::LateralClick,

        IPAChar::VoicelessLabialVelarFricative,
        IPAChar::VoicedLabialVelarApproximant,
        IPAChar::VoicedLabialPalatalApproximant,
        IPAChar::VoicelessPalatalVelarFricative,
        IPAChar::VoicedAlveolarLateralFlap,
        IPAChar::VoicelessAlveoloPalatalFricative,
        IPAChar::VoicedAlveoloPalatalFricative,

        IPAChar::VelarizedAlveolarLateralApproximant,
    ].into();

    /// Category of voiceless consonants.
    pub static ref VOICELESS_CONSONANTS: BTreeSet<IPAChar> = [
        IPAChar::VoicelessBilabialPlosive,
        IPAChar::VoicelessAlveolarPlosive,
        IPAChar::VoicelessRetroflexPlosive,
        IPAChar::VoicelessPalatalPlosive,
        IPAChar::VoicelessVelarPlosive,
        IPAChar::VoicelessUvularPlosive,
        IPAChar::PharyngealPlosive,
        IPAChar::GlottalPlosive,

        IPAChar::VoicelessPharyngealTrill,

        IPAChar::VoicelessBilabialFricative,
        IPAChar::VoicelessLabiodentalFricative,
        IPAChar::VoicelessDentalFricative,
        IPAChar::VoicelessAlveolarFricative,
        IPAChar::VoicelessPostalveolarFricative,
        IPAChar::VoicelessRetroflexFricative,
        IPAChar::VoicelessPalatalFricative,
        IPAChar::VoicelessVelarFricative,
        IPAChar::VoicelessUvularFricative,
        IPAChar::VoicelessPharyngealFricative,
        IPAChar::VoicelessGlottalFricative,

        IPAChar::VoicelessAlveolarLateralFricative,

        IPAChar::VoicelessBilabialImplosive,
        IPAChar::VoicelessAlveolarImplosive,
        IPAChar::VoicelessPalatalImplosive,
        IPAChar::VoicelessVelarImplosive,
        IPAChar::VoicelessUvularImplosive,

        IPAChar::BilabialClick,
        IPAChar::DentalClick,
        IPAChar::AlveolarClick,
        IPAChar::PalatalClick,
        IPAChar::LateralClick,

        IPAChar::VoicelessLabialVelarFricative,
        IPAChar::VoicelessPalatalVelarFricative,
        IPAChar::VoicelessAlveoloPalatalFricative,
    ].into();

    /// Category of voiced consonants.
    pub static ref VOICED_CONSONANTS: BTreeSet<IPAChar> = [
        IPAChar::VoicedBilabialPlosive,
        IPAChar::VoicedAlveolarPlosive,
        IPAChar::VoicedRetroflexPlosive,
        IPAChar::VoicedPalatalPlosive,
        IPAChar::VoicedVelarPlosive,
        IPAChar::VoicedUvularPlosive,

        IPAChar::VoicedBilabialNasal,
        IPAChar::VoicedLabiodentalNasal,
        IPAChar::VoicedAlveolarNasal,
        IPAChar::VoicedRetroflexNasal,
        IPAChar::VoicedPalatalNasal,
        IPAChar::VoicedVelarNasal,
        IPAChar::VoicedUvularNasal,

        IPAChar::VoicedBilabialTrill,
        IPAChar::VoicedAlveolarTrill,
        IPAChar::VoicedUvularTrill,
        IPAChar::VoicedPharyngealTrill,

        IPAChar::VoicedLabiodentalFlap,
        IPAChar::VoicedAlveolarFlap,
        IPAChar::VoicedRetroflexFlap,

        IPAChar::VoicedBilabialFricative,
        IPAChar::VoicedLabiodentalFricative,
        IPAChar::VoicedDentalFricative,
        IPAChar::VoicedAlveolarFricative,
        IPAChar::VoicedPostalveolarFricative,
        IPAChar::VoicedRetroflexFricative,
        IPAChar::VoicedPalatalFricative,
        IPAChar::VoicedVelarFricative,
        IPAChar::VoicedUvularFricative,
        IPAChar::VoicedPharygealFricative,
        IPAChar::VoicedGlottalFricative,

        IPAChar::VoicedAlveolarLateralFricative,

        IPAChar::VoicedLabiodentalApproximant,
        IPAChar::VoicedAlveolarApproximant,
        IPAChar::VoicedRetroflexApproximant,
        IPAChar::VoicedPalatalApproximant,
        IPAChar::VoicedVelarApproximant,

        IPAChar::VoicedAlveolarLateralApproximant,
        IPAChar::VoicedRetroflexLateralApproximant,
        IPAChar::VoicedPalatalLateralApproximant,
        IPAChar::VoicedVelarLateralApproximant,

        IPAChar::VoicedBilabialImplosive,
        IPAChar::VoicedAlveolarImplosive,
        IPAChar::VoicedPalatalImplosive,
        IPAChar::VoicedVelarImplosive,
        IPAChar::VoicedUvularImplosive,

        IPAChar::VoicedLabialVelarApproximant,
        IPAChar::VoicedLabialPalatalApproximant,
        IPAChar::VoicedAlveolarLateralFlap,
        IPAChar::VoicedAlveoloPalatalFricative,

        IPAChar::VelarizedAlveolarLateralApproximant,
    ].into();

    /// Category of obstruents.
    pub static ref OBSTRUENTS: BTreeSet<IPAChar> = [
        IPAChar::VoicelessBilabialPlosive,
        IPAChar::VoicedBilabialPlosive,
        IPAChar::VoicelessAlveolarPlosive,
        IPAChar::VoicedAlveolarPlosive,
        IPAChar::VoicelessRetroflexPlosive,
        IPAChar::VoicedRetroflexPlosive,
        IPAChar::VoicelessPalatalPlosive,
        IPAChar::VoicedPalatalPlosive,
        IPAChar::VoicelessVelarPlosive,
        IPAChar::VoicedVelarPlosive,
        IPAChar::VoicelessUvularPlosive,
        IPAChar::VoicedUvularPlosive,
        IPAChar::PharyngealPlosive,
        IPAChar::GlottalPlosive,

        IPAChar::VoicelessBilabialFricative,
        IPAChar::VoicedBilabialFricative,
        IPAChar::VoicelessLabiodentalFricative,
        IPAChar::VoicedLabiodentalFricative,
        IPAChar::VoicelessDentalFricative,
        IPAChar::VoicedDentalFricative,
        IPAChar::VoicelessAlveolarFricative,
        IPAChar::VoicedAlveolarFricative,
        IPAChar::VoicelessPostalveolarFricative,
        IPAChar::VoicedPostalveolarFricative,
        IPAChar::VoicelessRetroflexFricative,
        IPAChar::VoicedRetroflexFricative,
        IPAChar::VoicelessPalatalFricative,
        IPAChar::VoicedPalatalFricative,
        IPAChar::VoicelessVelarFricative,
        IPAChar::VoicedVelarFricative,
        IPAChar::VoicelessUvularFricative,
        IPAChar::VoicedUvularFricative,
        IPAChar::VoicelessPharyngealFricative,
        IPAChar::VoicedPharygealFricative,
        IPAChar::VoicelessGlottalFricative,
        IPAChar::VoicedGlottalFricative,

        IPAChar::VoicelessAlveolarLateralFricative,
        IPAChar::VoicedAlveolarLateralFricative,

        IPAChar::VoicelessBilabialImplosive,
        IPAChar::VoicedBilabialImplosive,
        IPAChar::VoicelessAlveolarImplosive,
        IPAChar::VoicedAlveolarImplosive,
        IPAChar::VoicelessPalatalImplosive,
        IPAChar::VoicedPalatalImplosive,
        IPAChar::VoicelessVelarImplosive,
        IPAChar::VoicedVelarImplosive,
        IPAChar::VoicelessUvularImplosive,
        IPAChar::VoicedUvularImplosive,

        IPAChar::BilabialClick,
        IPAChar::DentalClick,
        IPAChar::AlveolarClick,
        IPAChar::PalatalClick,
        IPAChar::LateralClick,

        IPAChar::VoicelessLabialVelarFricative,
        IPAChar::VoicelessPalatalVelarFricative,
        IPAChar::VoicelessAlveoloPalatalFricative,
        IPAChar::VoicedAlveoloPalatalFricative,
    ].into();

    /// Category of plosives or stops.
    pub static ref PLOSIVES: BTreeSet<IPAChar> = [
        IPAChar::VoicelessBilabialPlosive,
        IPAChar::VoicedBilabialPlosive,
        IPAChar::VoicelessAlveolarPlosive,
        IPAChar::VoicedAlveolarPlosive,
        IPAChar::VoicelessRetroflexPlosive,
        IPAChar::VoicedRetroflexPlosive,
        IPAChar::VoicelessPalatalPlosive,
        IPAChar::VoicedPalatalPlosive,
        IPAChar::VoicelessVelarPlosive,
        IPAChar::VoicedVelarPlosive,
        IPAChar::VoicelessUvularPlosive,
        IPAChar::VoicedUvularPlosive,
        IPAChar::PharyngealPlosive,
        IPAChar::GlottalPlosive,
    ].into();

    /// Category of fricatives.
    pub static ref FRICATIVES: BTreeSet<IPAChar> = [
        IPAChar::VoicelessBilabialFricative,
        IPAChar::VoicedBilabialFricative,
        IPAChar::VoicelessLabiodentalFricative,
        IPAChar::VoicedLabiodentalFricative,
        IPAChar::VoicelessDentalFricative,
        IPAChar::VoicedDentalFricative,
        IPAChar::VoicelessAlveolarFricative,
        IPAChar::VoicedAlveolarFricative,
        IPAChar::VoicelessPostalveolarFricative,
        IPAChar::VoicedPostalveolarFricative,
        IPAChar::VoicelessRetroflexFricative,
        IPAChar::VoicedRetroflexFricative,
        IPAChar::VoicelessPalatalFricative,
        IPAChar::VoicedPalatalFricative,
        IPAChar::VoicelessVelarFricative,
        IPAChar::VoicedVelarFricative,
        IPAChar::VoicelessUvularFricative,
        IPAChar::VoicedUvularFricative,
        IPAChar::VoicelessPharyngealFricative,
        IPAChar::VoicedPharygealFricative,
        IPAChar::VoicelessGlottalFricative,
        IPAChar::VoicedGlottalFricative,

        IPAChar::VoicelessAlveolarLateralFricative,
        IPAChar::VoicedAlveolarLateralFricative,

        IPAChar::VoicelessLabialVelarFricative,
        IPAChar::VoicelessPalatalVelarFricative,
        IPAChar::VoicelessAlveoloPalatalFricative,
        IPAChar::VoicedAlveoloPalatalFricative,
    ].into();

    /// Category of resonants.
    pub static ref RESONANTS: BTreeSet<IPAChar> = [
        IPAChar::VoicedBilabialNasal,
        IPAChar::VoicedLabiodentalNasal,
        IPAChar::VoicedAlveolarNasal,
        IPAChar::VoicedRetroflexNasal,
        IPAChar::VoicedPalatalNasal,
        IPAChar::VoicedVelarNasal,
        IPAChar::VoicedUvularNasal,

        IPAChar::VoicedBilabialTrill,
        IPAChar::VoicedAlveolarTrill,
        IPAChar::VoicedUvularTrill,
        IPAChar::VoicelessPharyngealTrill,
        IPAChar::VoicedPharyngealTrill,

        IPAChar::VoicedLabiodentalFlap,
        IPAChar::VoicedAlveolarFlap,
        IPAChar::VoicedRetroflexFlap,

        IPAChar::VoicedLabiodentalApproximant,
        IPAChar::VoicedAlveolarApproximant,
        IPAChar::VoicedRetroflexApproximant,
        IPAChar::VoicedPalatalApproximant,
        IPAChar::VoicedVelarApproximant,

        IPAChar::VoicedAlveolarLateralApproximant,
        IPAChar::VoicedRetroflexLateralApproximant,
        IPAChar::VoicedPalatalLateralApproximant,
        IPAChar::VoicedVelarLateralApproximant,

        IPAChar::VoicedLabialVelarApproximant,
        IPAChar::VoicedLabialPalatalApproximant,
        IPAChar::VoicedAlveolarLateralFlap,

        IPAChar::VelarizedAlveolarLateralApproximant,
    ].into();

    /// Category of nasals.
    pub static ref NASALS: BTreeSet<IPAChar> = [
        IPAChar::VoicedBilabialNasal,
        IPAChar::VoicedLabiodentalNasal,
        IPAChar::VoicedAlveolarNasal,
        IPAChar::VoicedRetroflexNasal,
        IPAChar::VoicedPalatalNasal,
        IPAChar::VoicedVelarNasal,
        IPAChar::VoicedUvularNasal,
    ].into();

    /// Category of approximants.
    pub static ref APPROXIMANTS: BTreeSet<IPAChar> = [
        IPAChar::VoicedLabiodentalApproximant,
        IPAChar::VoicedAlveolarApproximant,
        IPAChar::VoicedRetroflexApproximant,
        IPAChar::VoicedPalatalApproximant,
        IPAChar::VoicedVelarApproximant,

        IPAChar::VoicedAlveolarLateralApproximant,
        IPAChar::VoicedRetroflexLateralApproximant,
        IPAChar::VoicedPalatalLateralApproximant,
        IPAChar::VoicedVelarLateralApproximant,

        IPAChar::VoicedLabialVelarApproximant,
        IPAChar::VoicedLabialPalatalApproximant,

        IPAChar::VelarizedAlveolarLateralApproximant,
    ].into();

    /// Category of trills.
    pub static ref TRILLS: BTreeSet<IPAChar> = [
        IPAChar::VoicedBilabialTrill,
        IPAChar::VoicedAlveolarTrill,
        IPAChar::VoicedUvularTrill,
        IPAChar::VoicelessPharyngealTrill,
        IPAChar::VoicedPharyngealTrill,
    ].into();

    /// Category of taps or flaps.
    pub static ref FLAPS: BTreeSet<IPAChar> = [
        IPAChar::VoicedLabiodentalFlap,
        IPAChar::VoicedAlveolarFlap,
        IPAChar::VoicedRetroflexFlap,
        IPAChar::VoicedAlveolarLateralFlap,
    ].into();

    /// Category of vowels.
    pub static ref VOWELS: BTreeSet<IPAChar> = [
        IPAChar::CloseFrontUnroundedVowel,
        IPAChar::CloseFrontRoundedVowel,
        IPAChar::CloseCentralUnroundedVowel,
        IPAChar::CloseCentralRoundedVowel,
        IPAChar::CloseBackUnroundedVowel,
        IPAChar::CloseBackRoundedVowel,

        IPAChar::NearCloseFrontUnroundedVowel,
        IPAChar::NearCloseFrontRoundedVowel,
        IPAChar::NearCloseBackRoundedVowel,

        IPAChar::CloseMidFrontUnroundedVowel,
        IPAChar::CloseMidFrontRoundedVowel,
        IPAChar::CloseMidCentralUnroundedVowel,
        IPAChar::CloseMidCentralRoundedVowel,
        IPAChar::CloseMidBackUnroundedVowel,
        IPAChar::CloseMidBackRoundedVowel,

        IPAChar::MidCentralVowel,
        IPAChar::RhoticVowel,

        IPAChar::OpenMidFrontUnroundedVowel,
        IPAChar::OpenMidFrontRoundedVowel,
        IPAChar::OpenMidCentralUnroundedVowel,
        IPAChar::OpenMidCentralRoundedVowel,
        IPAChar::OpenMidBackUnroundedVowel,
        IPAChar::OpenMidBackRoundedVowel,

        IPAChar::NearOpenFrontUnroundedVowel,
        IPAChar::NearOpenCentralVowel,

        IPAChar::OpenFrontUnroundedVowel,
        IPAChar::OpenFrontRoundedVowel,
        IPAChar::OpenBackUnroundedVowel,
        IPAChar::OpenBackRoundedVowel,
    ].into();

    /// Category of unrounded vowels.
    pub static ref UNROUNDED_VOWELS: BTreeSet<IPAChar> = [
        IPAChar::CloseFrontUnroundedVowel,
        IPAChar::CloseCentralUnroundedVowel,
        IPAChar::CloseBackUnroundedVowel,

        IPAChar::NearCloseFrontUnroundedVowel,

        IPAChar::CloseMidFrontUnroundedVowel,
        IPAChar::CloseMidCentralUnroundedVowel,
        IPAChar::CloseMidBackUnroundedVowel,

        IPAChar::MidCentralVowel,
        IPAChar::RhoticVowel,

        IPAChar::OpenMidFrontUnroundedVowel,
        IPAChar::OpenMidCentralUnroundedVowel,
        IPAChar::OpenMidBackUnroundedVowel,

        IPAChar::NearOpenFrontUnroundedVowel,
        IPAChar::NearOpenCentralVowel,

        IPAChar::OpenFrontUnroundedVowel,
        IPAChar::OpenBackUnroundedVowel,
    ].into();

    /// Category of rounded vowels.
    pub static ref ROUNDED_VOWELS: BTreeSet<IPAChar> = [
        IPAChar::CloseFrontRoundedVowel,
        IPAChar::CloseCentralRoundedVowel,
        IPAChar::CloseBackRoundedVowel,

        IPAChar::NearCloseFrontRoundedVowel,
        IPAChar::NearCloseBackRoundedVowel,

        IPAChar::CloseMidFrontRoundedVowel,
        IPAChar::CloseMidCentralRoundedVowel,
        IPAChar::CloseMidBackRoundedVowel,

        IPAChar::OpenMidFrontRoundedVowel,
        IPAChar::OpenMidCentralRoundedVowel,
        IPAChar::OpenMidBackRoundedVowel,

        IPAChar::OpenFrontRoundedVowel,
        IPAChar::OpenBackRoundedVowel,
    ].into();
}
