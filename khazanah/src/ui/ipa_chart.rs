use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;
use once_cell::sync::Lazy;

use conlang::ipa;

use crate::ui;

pub struct ChartTitle<T> {
    pub titles: Vec<String>,
    pub values: Vec<T>,
}

#[macro_export]
macro_rules! chart_titles {
    ($($title:literal => $value:expr),* $(,)?) => {
        $crate::ui::ipa_chart::ChartTitle {
            titles: vec![
                $($title.to_string()),*
            ],
            values: vec![
                $($value),*
            ],
        }
    };
}

pub static PULMONIC_CONSONANT_MANNER_TITLES: Lazy<ChartTitle<ipa::MannerOfArticulation>> =
    Lazy::new(|| {
        use ipa::FricativeVariant::*;
        use ipa::MannerOfArticulation::*;
        chart_titles! {
            "Nasal" => Nasal,
            "Plosive" => Plosive,
            "Sibilant fricative" => Fricative(Sibilant),
            "Non-sibilant fricative" => Fricative(NonSibilant),
            "Sibilant affricate" => Affricate(Sibilant),
            "Non-sibilant affricate" => Affricate(NonSibilant),
            "Approximant" => Approximant,
            "Tap/flap" => Flap,
            "Trill" => Trill,
            "Lateral fricative" => LateralFricative,
            "Lateral affricate" => LateralAffricate,
            "Lateral approximant" => LateralApproximant,
            "Lateral tap/flap" => LateralFlap,
        }
    });

pub static PULMONIC_CONSONANT_PLACE_TITLES: Lazy<ChartTitle<ipa::PlaceOfArticulation>> =
    Lazy::new(|| {
        use ipa::PlaceOfArticulation::*;
        chart_titles! {
            "BL" => Bilabial,
            "LD" => Labiodental,
            "D" => Dental,
            "A" => Alveolar,
            "PA" => PostAlveolar,
            "RF" => Retroflex,
            "P" => Palatal,
            "V" => Velar,
            "U" => Uvular,
            "EG" => Pharyngeal,
            "G" => Glottal,
        }
    });

pub static COARTICULATED_CONSONANT_MANNER_TITLES: Lazy<ChartTitle<ipa::MannerOfArticulation>> =
    Lazy::new(|| {
        use ipa::FricativeVariant::*;
        use ipa::MannerOfArticulation::*;
        chart_titles! {
            "Nasal" => Nasal,
            "Plosive" => Plosive,
            "Fricative" => Fricative(NonSibilant),
            "Approximant" => Approximant,
        }
    });

pub static COARTICULATED_CONSONANT_PLACE_TITLES: Lazy<ChartTitle<ipa::PlaceOfArticulation>> =
    Lazy::new(|| {
        use ipa::PlaceOfArticulation::*;
        chart_titles! {
            "Labial-alveolar" => LabialAlveolar,
            "Labial-palatal" => LabialPalatal,
            "Labial-velar" => LabialVelar,
            "Uvular-pharyngeal" => UvularPharyngeal,
        }
    });

pub static EJECTIVE_CONSONANT_MANNER_TITLES: Lazy<ChartTitle<ipa::MannerOfArticulation>> =
    Lazy::new(|| {
        use ipa::FricativeVariant::*;
        use ipa::MannerOfArticulation::*;
        chart_titles! {
            "Plosive" => Plosive,
            "Sibilant fricative" => Fricative(Sibilant),
            "Non-sibilant fricative" => Fricative(NonSibilant),
            "Sibilant affricate" => Affricate(Sibilant),
            "Non-sibilant affricate" => Affricate(NonSibilant),
            "Lateral fricative" => LateralFricative,
            "Lateral affricate" => LateralAffricate,
        }
    });

pub static EJECTIVE_CONSONANT_PLACE_TITLES: Lazy<ChartTitle<ipa::PlaceOfArticulation>> =
    Lazy::new(|| {
        use ipa::PlaceOfArticulation::*;
        chart_titles! {
            "BL" => Bilabial,
            "LD" => Labiodental,
            "D" => Dental,
            "A" => Alveolar,
            "PA" => PostAlveolar,
            "RF" => Retroflex,
            "P" => Palatal,
            "V" => Velar,
            "U" => Uvular,
            "EG" => Pharyngeal,
        }
    });

pub static CLICK_CONSONANT_MANNER_PLACE_TITLES: Lazy<ChartTitle<ipa::ClickMannerOfArticulation>> =
    Lazy::new(|| {
        use ipa::ClickMannerOfArticulation::*;
        chart_titles! {
            "Tenuis" => Tenuis,
            "Voiced" => Voiced,
            "Nasal" => Nasal,
            "Tenuis lateral" => TenuisLateral,
            "Voiced lateral" => VoicedLateral,
            "Nasal lateral" => NasalLateral,
        }
    });

pub static CLICK_CONSONANT_PLACE_TITLES: Lazy<ChartTitle<ipa::PlaceOfArticulation>> =
    Lazy::new(|| {
        use ipa::PlaceOfArticulation::*;
        chart_titles! {
            "BL" => Bilabial,
            "D" => Dental,
            "A" => Alveolar,
            "RF" => Retroflex,
            "P" => Palatal,
        }
    });

pub static IMPLOSIVE_CONSONANT_PLACE_TITLES: Lazy<ChartTitle<ipa::PlaceOfArticulation>> =
    Lazy::new(|| {
        use ipa::PlaceOfArticulation::*;
        chart_titles! {
            "BL" => Bilabial,
            "A" => Alveolar,
            "RF" => Retroflex,
            "P" => Palatal,
            "V" => Velar,
            "U" => Uvular,
        }
    });

pub static CONSONANT_PHONATION_TITLES: Lazy<ChartTitle<ipa::Phonation>> = Lazy::new(|| {
    use ipa::Phonation::*;
    chart_titles! {
        "Voiceless" => Voiceless,
        "Voiced" => Voiced,
    }
});

pub static VOWEL_HEIGHT_TITLES: Lazy<ChartTitle<ipa::VowelHeight>> = Lazy::new(|| {
    use ipa::VowelHeight::*;
    chart_titles! {
        "Close" => Close,
        "Near-close" => NearClose,
        "Close-mid" => CloseMid,
        "Mid" => Mid,
        "Open-mid" => OpenMid,
        "Near-open" => NearOpen,
        "Open" => Open,
    }
});

pub static VOWEL_BACKNESS_TITLES: Lazy<ChartTitle<ipa::VowelBackness>> = Lazy::new(|| {
    use ipa::VowelBackness::*;
    chart_titles! {
        "Front" => Front,
        "Central" => Central,
        "Back" => Back,
    }
});

pub static VOWEL_ROUNDING_TITLES: Lazy<ChartTitle<ipa::VowelRounding>> = Lazy::new(|| {
    use ipa::VowelRounding::*;
    chart_titles! {
        "Unrounded" => Unrounded,
        "Rounded" => Rounded,
    }
});

pub static UNIT_TITLE: Lazy<ChartTitle<()>> = Lazy::new(|| {
    chart_titles! {
        "" => (),
    }
});

mod imp {
    use gtk::glib::{subclass::Signal, FromVariant};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/ipa_chart.ui")]
    pub struct IpaChart {
        #[template_child]
        pub chart: TemplateChild<gtk::Grid>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IpaChart {
        const NAME: &'static str = "KhzIpaChart";
        type Type = super::IpaChart;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action("ipa.symbol", Some("s"), |widget, _, v| {
                if let Some(v) = v.and_then(String::from_variant) {
                    widget.emit_by_name::<()>("symbol-selected", &[&v]);
                }
            })
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for IpaChart {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("symbol-selected")
                    .param_types([String::static_type()])
                    .build()]
            });
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for IpaChart {}

    impl BinImpl for IpaChart {}
}

glib::wrapper! {
    /// IPA chart.
    pub struct IpaChart(ObjectSubclass<imp::IpaChart>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl IpaChart {
    /// Populate chart.
    pub fn populate_chart3<T1, T2, T3, F>(
        &self,
        title1: &ChartTitle<T1>,
        title2: &ChartTitle<T2>,
        title3: &ChartTitle<T3>,
        f: F,
    ) where
        T1: Copy,
        T2: Copy,
        T3: Copy,
        F: Fn(T1, T2, T3) -> ipa::Ipa + 'static,
    {
        let chart = self.imp().chart.get();
        let n1 = title1.values.len() as i32;
        let n2 = title2.values.len() as i32;
        let n3 = title3.values.len() as i32;

        {
            let widget = gtk::Button::builder()
                .css_classes(["table-title", "corner-top-left"])
                .build();
            chart.attach(&widget, 0, 0, 1, 1);
        }

        for c in 0..n1 {
            let label = gtk::Label::builder()
                .label(title1.titles[c as usize].as_str())
                .build();
            let widget = gtk::Button::builder()
                .child(&label)
                .css_classes(["table-title"])
                .build();
            if c == n1 - 1 {
                widget.add_css_class("corner-top-right");
            }
            chart.attach(&widget, c * n3 + 1, 0, n3, 1);
        }

        for r in 0..n2 {
            let label = gtk::Label::builder()
                .label(title2.titles[r as usize].as_str())
                .halign(gtk::Align::Start)
                .build();
            let widget = gtk::Button::builder()
                .child(&label)
                .css_classes(["table-title"])
                .build();
            if r == n2 - 1 {
                widget.add_css_class("corner-bottom-left");
            }
            chart.attach(&widget, 0, r + 1, 1, 1);
        }

        for c in 0..n1 {
            let v1 = title1.values[c as usize];
            for r in 0..n2 {
                let v2 = title2.values[r as usize];
                for s in 0..n3 {
                    let v3 = title3.values[s as usize];

                    let ipa_sym = f(v1, v2, v3);
                    let sym = ipa_sym.to_str();
                    let widget = gtk::Button::builder()
                        .sensitive(sym.is_some())
                        .label(sym.unwrap_or_default())
                        .css_classes(["symbol"])
                        .build();
                    if let Some(sym) = sym {
                        let tooltip: String = ipa_sym
                            .to_string()
                            .char_indices()
                            .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                            .collect();
                        widget.set_action_name(Some("ipa.symbol"));
                        widget.set_action_target(Some(sym.to_variant()));
                        widget.set_tooltip_text(Some(&tooltip));
                    }
                    if s == n3 - 1 {
                        widget.add_css_class("symbol-right");
                    }
                    if c == n1 - 1 && r == n2 - 1 && s == n3 - 1 {
                        widget.add_css_class("corner-bottom-right");
                    }
                    chart.attach(&widget, c * n3 + s + 1, r + 1, 1, 1);
                }
            }
        }
    }

    // Populates chart with pulmonic consonants.
    pub fn populate_pulmonic_consonants(&self) {
        self.populate_chart3(
            &PULMONIC_CONSONANT_PLACE_TITLES,
            &PULMONIC_CONSONANT_MANNER_TITLES,
            &CONSONANT_PHONATION_TITLES,
            |p, m, v| ipa::Ipa::Consonant(v, p, m),
        );
    }

    // Populates chart with coarticulated consonants.
    pub fn populate_coarticulated_consonants(&self) {
        self.populate_chart3(
            &COARTICULATED_CONSONANT_PLACE_TITLES,
            &COARTICULATED_CONSONANT_MANNER_TITLES,
            &CONSONANT_PHONATION_TITLES,
            |p, m, v| ipa::Ipa::Consonant(v, p, m),
        );
    }

    // Populates chart with ejective consonants.
    pub fn populate_ejective_consonants(&self) {
        self.populate_chart3(
            &EJECTIVE_CONSONANT_PLACE_TITLES,
            &EJECTIVE_CONSONANT_MANNER_TITLES,
            &UNIT_TITLE,
            |p, m, _| ipa::Ipa::EjectiveConsonant(p, m),
        );
    }

    // Populates chart with implosive consonants.
    pub fn populate_implosive_consonants(&self) {
        self.populate_chart3(
            &IMPLOSIVE_CONSONANT_PLACE_TITLES,
            &CONSONANT_PHONATION_TITLES,
            &UNIT_TITLE,
            |p, v, _| ipa::Ipa::ImplosiveConsonant(v, p),
        );
    }

    // Populates chart with click consonants.
    pub fn populate_click_consonants(&self) {
        self.populate_chart3(
            &CLICK_CONSONANT_PLACE_TITLES,
            &CLICK_CONSONANT_MANNER_PLACE_TITLES,
            &UNIT_TITLE,
            |p, m, _| ipa::Ipa::ClickConsonant(p, m),
        );
    }

    // Populates chart with vowels.
    pub fn populate_vowels(&self) {
        self.populate_chart3(
            &VOWEL_BACKNESS_TITLES,
            &VOWEL_HEIGHT_TITLES,
            &VOWEL_ROUNDING_TITLES,
            |b, h, r| ipa::Ipa::Vowel(h, b, r),
        );
    }
}

impl ui::View for IpaChart {}
