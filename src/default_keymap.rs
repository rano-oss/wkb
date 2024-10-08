use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref DEFAULT_LEVEL1_MAP: HashMap<u32, char> = [
        (1, '\u{1b}'),
        (14, '\u{8}'),
        (15,'\t'),
        (28,'\r'),
        (43, '\\'),
        (57, ' '),
        (86,'<'),
        (101,'\n'),
        (111, '\u{7f}'),
        (117,'='),
        (118, '±'),
        (179, '('),
        (180,')'),
        (434, '$'),
        (435, '€'),
        (55, '*'),
        (74, '-'),
        (78, '+'),
        (96,'\r'),
        (98, '/'),
        (121,'.'),
    ].into();
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_LEVEL2_MAP: HashMap<u32, char> = [
        (1, '\u{1b}'),
        (14, '\u{8}'),
        (28,'\r'),
        (43, '|'),
        (57,'\u{20}'),
        (86,'>'),
        (96, '\u{d}'),
        (101,'\n'),
        (111, '\u{7f}'),
        (117,'='),
        (118, '±'),
        (179, '('),
        (180,')'),
        (434, '$'),
        (435, '€'),
        (55, '*'),
        (71, '7'),
        (72, '8'),
        (73, '9'),
        (74, '-'),
        (75, '4'),
        (76, '5'),
        (77, '6'),
        (78, '+'),
        (79, '1'),
        (80, '2'),
        (81, '3'),
        (82, '0'),
        (83, '.'),
        (98, '/'),
        (121,'.'),
    ].into();
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_LEVEL3_MAP: HashMap<u32, char> = [
        (86, '|'),
        (55, '*'),
        (74, '-'),
        (78, '+'),
        (98, '/'),
    ].into();
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_LEVEL4_MAP: HashMap<u32, char> = [
        (86, '¦'),
        (55, '*'),
        (74, '-'),
        (78, '+'),
        (98, '/'),
    ].into();
}
