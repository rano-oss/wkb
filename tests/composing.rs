use test_case::test_matrix;
use wkb::{self, composer::Composer, ComposeState};

#[test_matrix([
    "af", "al", "am", "ara", "at", "au", "az", "ba", "bd", "be", "bg", "br", "bt", "bw", "by",
    "ca", "cd", "ch", "cm", "cn", "cz", "de", "dk", "dz", "ee", "eg", "epo", "es", "et", "eu",
    "fi", "fo", "fr", "gb", "ge", "gh", "gn", "gr", "hr", "hu", "id", "ie", "il", "in", "iq",
    "ir", "is", "it", "jp", "ke", "kg", "kh", "kr", "kz", "la", "latam", "lk", "lt", "lv",
    "ma", "md", "me", "mk", "ml", "mm", "mn", "mt", "mv", "my", "ng", "nl", "no", "np", "nz",
    "ph", "pk", "pl", "pt", "ro", "rs", "ru", "se", "si", "sk", "sn", "tg", "th", "tj", "tm",
    "tr", "tw", "tz", "ua", "us", "uz", "vn", "za"
])]
fn composing(locale: &str) {
    let compose_file_subpath = wkb::xkb::resolve_compose_file(locale);
    assert!(
        compose_file_subpath.is_some(),
        "resolve_compose_file('{}') returned None",
        locale
    );

    let wkb = wkb::WKB::new_from_names(locale.to_string(), None);

    // Verify that the regular composer was loaded and is operational
    // by feeding a known compose starter (combining acute accent)
    // that exists in virtually every Compose file.
    let mut regular = wkb.regular_composer.clone();
    let result = regular.feed('\u{0301}');
    assert!(
        !matches!(result, ComposeState::Idle(_)),
        "{}: regular composer should recognise combining acute as a compose starter",
        locale
    );
}
