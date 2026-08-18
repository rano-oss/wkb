//! Compact checked-in parser automaton. No generation step is used.
#![allow(non_upper_case_globals)]

pub(crate) const NTOKENS: u8 = 66;

pub(crate) type Symbol = u8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Action {
    Shift(u16),
    Reduce(u16),
    Accept,
    Error,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Transition(Symbol, i16);

#[derive(Clone, Copy, Debug)]
pub(crate) struct State(u16, u8, i16);

#[derive(Clone, Copy, Debug)]
pub(crate) struct Rule(Symbol, u8, u16);

const ACCEPT: i16 = i16::MAX;

fn decode_action(encoded: i16) -> Action {
    match encoded {
        ACCEPT => Action::Accept,
        0 => Action::Error,
        n if n > 0 => Action::Shift(n as u16),
        n => Action::Reduce((-n) as u16),
    }
}

impl State {
    pub(crate) fn transitions(&self) -> &'static [Transition] {
        let start = self.0 as usize;
        &TRANSITIONS[start..start + self.1 as usize]
    }
    pub(crate) fn explicit_action(&self, symbol: Symbol) -> Option<Action> {
        self.transitions()
            .iter()
            .find_map(|entry| (entry.0 == symbol).then(|| decode_action(entry.1)))
    }
    pub(crate) fn has_terminal_transitions(&self) -> bool {
        self.transitions().iter().any(|entry| entry.0 < NTOKENS)
    }
    pub(crate) fn default_action(&self) -> Action {
        decode_action(self.2)
    }
}

impl Rule {
    pub(crate) fn rhs_len(&self) -> u8 {
        self.1
    }
    pub(crate) fn next_state(&self, state: u16) -> u16 {
        match STATES[state as usize].explicit_action(self.0) {
            Some(Action::Shift(next)) => next,
            None => self.2,
            Some(action) => unreachable!("invalid goto action: {action:?}"),
        }
    }
}

macro_rules! symbols { ($($name:ident = $value:literal),* $(,)?) => { $(pub(crate) const $name: Symbol = $value;)* }; }
#[rustfmt::skip]
symbols! {
    SYM_EOF = 0, SYM_XKB_KEYMAP = 4, SYM_XKB_KEYCODES = 5, SYM_XKB_TYPES = 6, SYM_XKB_SYMBOLS = 7, SYM_XKB_COMPATMAP = 8, SYM_XKB_GEOMETRY = 9, SYM_XKB_SEMANTICS = 10, SYM_XKB_LAYOUT = 11, SYM_INCLUDE = 12, SYM_OVERRIDE = 13, SYM_AUGMENT = 14, SYM_REPLACE = 15,
    SYM_ALTERNATE = 16, SYM_VIRTUAL_MODS = 17, SYM_TYPE = 18, SYM_INTERPRET = 19, SYM_ACTION_TOK = 20, SYM_KEY = 21, SYM_ALIAS = 22, SYM_GROUP = 23, SYM_MODIFIER_MAP = 24, SYM_INDICATOR = 25, SYM_SHAPE = 26, SYM_KEYS = 27, SYM_ROW = 28, SYM_SECTION = 29, SYM_OVERLAY = 30, SYM_TEXT = 31,
    SYM_OUTLINE = 32, SYM_SOLID = 33, SYM_LOGO = 34, SYM_VIRTUAL = 35, SYM_EQUALS = 36, SYM_PLUS = 37, SYM_MINUS = 38, SYM_DIVIDE = 39, SYM_TIMES = 40, SYM_OBRACE = 41, SYM_CBRACE = 42, SYM_OPAREN = 43, SYM_CPAREN = 44, SYM_OBRACKET = 45, SYM_CBRACKET = 46, SYM_DOT = 47,
    SYM_COMMA = 48, SYM_SEMI = 49, SYM_EXCLAM = 50, SYM_INVERT = 51, SYM_STRING = 52, SYM_DECIMAL_DIGIT = 53, SYM_INTEGER = 54, SYM_FLOAT = 55, SYM_IDENT = 56, SYM_KEYNAME = 57, SYM_PARTIAL = 58, SYM_DEFAULT = 59, SYM_HIDDEN = 60, SYM_ALPHANUMERIC_KEYS = 61, SYM_MODIFIER_KEYS = 62, SYM_KEYPAD_KEYS = 63,
    SYM_FUNCTION_KEYS = 64, SYM_ALTERNATE_GROUP = 65, SYM_accept = 66, SYM_XkbFile = 67, SYM_XkbCompositeMap = 68, SYM_XkbCompositeType = 69, SYM_XkbMapConfigList = 70, SYM_XkbMapConfig = 71, SYM_FileType = 72, SYM_OptFlags = 73, SYM_Flags = 74, SYM_Flag = 75, SYM_DeclList = 76, SYM_Decl = 77, SYM_VarDecl = 78, SYM_KeyNameDecl = 79,
    SYM_KeyAliasDecl = 80, SYM_VModDecl = 81, SYM_VModDefList = 82, SYM_VModDef = 83, SYM_InterpretDecl = 84, SYM_InterpretMatch = 85, SYM_VarDeclList = 86, SYM_KeyTypeDecl = 87, SYM_SymbolsDecl = 88, SYM_OptSymbolsBody = 89, SYM_SymbolsBody = 90, SYM_SymbolsVarDecl = 91, SYM_MultiKeySymOrActionList = 92, SYM_NoSymbolOrActionList = 93, SYM_GroupCompatDecl = 94, SYM_ModMapDecl = 95,
    SYM_KeyOrKeySymList = 96, SYM_KeyOrKeySym = 97, SYM_LedMapDecl = 98, SYM_LedNameDecl = 99, SYM_UnknownDecl = 100, SYM_UnknownCompoundStatementDecl = 101, SYM_ShapeDecl = 102, SYM_SectionDecl = 103, SYM_SectionBody = 104, SYM_SectionBodyItem = 105, SYM_RowBody = 106, SYM_RowBodyItem = 107, SYM_Keys = 108, SYM_Key = 109, SYM_OverlayDecl = 110, SYM_OverlayKeyList = 111,
    SYM_OverlayKey = 112, SYM_OutlineList = 113, SYM_OutlineInList = 114, SYM_CoordList = 115, SYM_Coord = 116, SYM_DoodadDecl = 117, SYM_DoodadType = 118, SYM_FieldSpec = 119, SYM_Element = 120, SYM_OptMergeMode = 121, SYM_MergeMode = 122, SYM_ExprList = 123, SYM_Expr = 124, SYM_Term = 125, SYM_MultiActionList = 126, SYM_ActionList = 127,
    SYM_NonEmptyActions = 128, SYM_Actions = 129, SYM_Action = 130, SYM_Lhs = 131, SYM_OptTerminal = 132, SYM_Terminal = 133, SYM_MultiKeySymList = 134, SYM_KeySymList = 135, SYM_NonEmptyKeySyms = 136, SYM_KeySyms = 137, SYM_KeySym = 138, SYM_KeySymLit = 139, SYM_SignedNumber = 140, SYM_Number = 141, SYM_Float = 142, SYM_Integer = 143,
    SYM_KeyCode = 144, SYM_Ident = 145, SYM_String = 146, SYM_OptMapName = 147, SYM_MapName = 148,
}

macro_rules! tr {
    ($symbol:ident, s $state:literal) => {
        Transition($symbol, $state)
    };
    ($symbol:ident, g $state:literal) => {
        Transition($symbol, $state)
    };
    ($symbol:ident, r $rule:literal) => {
        Transition($symbol, -$rule)
    };
}
macro_rules! st {
    ($start:literal, $len:literal, e) => {
        State($start, $len, 0)
    };
    ($start:literal, $len:literal, a) => {
        State($start, $len, ACCEPT)
    };
    ($start:literal, $len:literal, r $rule:literal) => {
        State($start, $len, -$rule)
    };
}

#[rustfmt::skip]
static TRANSITIONS: &[Transition; 1385] = &[
    tr!(SYM_EOF, s 1), tr!(SYM_PARTIAL, s 2), tr!(SYM_DEFAULT, s 3), tr!(SYM_HIDDEN, s 4), tr!(SYM_ALPHANUMERIC_KEYS, s 5), tr!(SYM_MODIFIER_KEYS, s 6), tr!(SYM_KEYPAD_KEYS, s 7), tr!(SYM_FUNCTION_KEYS, s 8), tr!(SYM_ALTERNATE_GROUP, s 9), tr!(SYM_EOF, s 16), tr!(SYM_XKB_KEYMAP, s 17), tr!(SYM_XKB_KEYCODES, s 18), tr!(SYM_XKB_TYPES, s 19), tr!(SYM_XKB_SYMBOLS, s 20), tr!(SYM_XKB_COMPATMAP, s 21), tr!(SYM_XKB_GEOMETRY, s 22),
    tr!(SYM_XKB_SEMANTICS, s 23), tr!(SYM_XKB_LAYOUT, s 24), tr!(SYM_PARTIAL, s 2), tr!(SYM_DEFAULT, s 3), tr!(SYM_HIDDEN, s 4), tr!(SYM_ALPHANUMERIC_KEYS, s 5), tr!(SYM_MODIFIER_KEYS, s 6), tr!(SYM_KEYPAD_KEYS, s 7), tr!(SYM_FUNCTION_KEYS, s 8), tr!(SYM_ALTERNATE_GROUP, s 9), tr!(SYM_Flag, g 27), tr!(SYM_STRING, s 28), tr!(SYM_STRING, s 28), tr!(SYM_OptMapName, g 31), tr!(SYM_OBRACE, s 32), tr!(SYM_OBRACE, s 33),
    tr!(SYM_CBRACE, s 36), tr!(SYM_PARTIAL, s 2), tr!(SYM_DEFAULT, s 3), tr!(SYM_HIDDEN, s 4), tr!(SYM_ALPHANUMERIC_KEYS, s 5), tr!(SYM_MODIFIER_KEYS, s 6), tr!(SYM_KEYPAD_KEYS, s 7), tr!(SYM_FUNCTION_KEYS, s 8), tr!(SYM_ALTERNATE_GROUP, s 9), tr!(SYM_XkbMapConfig, g 37), tr!(SYM_OptFlags, g 38), tr!(SYM_INCLUDE, s 39), tr!(SYM_OVERRIDE, s 40), tr!(SYM_AUGMENT, s 41), tr!(SYM_REPLACE, s 42), tr!(SYM_ALTERNATE, s 43),
    tr!(SYM_CBRACE, s 44), tr!(SYM_SEMI, s 48), tr!(SYM_XKB_KEYCODES, s 18), tr!(SYM_XKB_TYPES, s 19), tr!(SYM_XKB_SYMBOLS, s 20), tr!(SYM_XKB_COMPATMAP, s 21), tr!(SYM_XKB_GEOMETRY, s 22), tr!(SYM_SEMI, s 49), tr!(SYM_VIRTUAL_MODS, s 50), tr!(SYM_TYPE, s 51), tr!(SYM_INTERPRET, s 52), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 54), tr!(SYM_ALIAS, s 55), tr!(SYM_GROUP, s 56), tr!(SYM_MODIFIER_MAP, s 57),
    tr!(SYM_INDICATOR, s 58), tr!(SYM_SHAPE, s 59), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 61), tr!(SYM_TEXT, s 62), tr!(SYM_OUTLINE, s 63), tr!(SYM_SOLID, s 64), tr!(SYM_LOGO, s 65), tr!(SYM_VIRTUAL, s 66), tr!(SYM_EXCLAM, s 67), tr!(SYM_IDENT, s 68), tr!(SYM_KEYNAME, s 69), tr!(SYM_DEFAULT, s 70), tr!(SYM_VarDecl, g 71), tr!(SYM_LedMapDecl, g 80), tr!(SYM_DoodadDecl, g 86),
    tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 90), tr!(SYM_Ident, g 91), tr!(SYM_STRING, s 92), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_Ident, g 96), tr!(SYM_STRING, s 97), tr!(SYM_String, g 98), tr!(SYM_SECTION, s 99), tr!(SYM_STRING, s 100), tr!(SYM_DECIMAL_DIGIT, s 101), tr!(SYM_INTEGER, s 102), tr!(SYM_IDENT, s 103), tr!(SYM_KeySym, g 105), tr!(SYM_KEYNAME, s 107),
    tr!(SYM_KEYNAME, s 108), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_Integer, g 111), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_Ident, g 112), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_Integer, g 113), tr!(SYM_String, g 114), tr!(SYM_STRING, s 97), tr!(SYM_String, g 115), tr!(SYM_STRING, s 97), tr!(SYM_String, g 116),
    tr!(SYM_STRING, r 124), tr!(SYM_INDICATOR, s 117), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_Ident, g 118), tr!(SYM_OBRACE, r 181), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_KEYNAME, s 120), tr!(SYM_Terminal, g 122), tr!(SYM_EQUALS, s 126), tr!(SYM_STRING, s 97), tr!(SYM_String, g 127), tr!(SYM_OBRACKET, s 128),
    tr!(SYM_DOT, s 129), tr!(SYM_EQUALS, s 130), tr!(SYM_SEMI, s 131), tr!(SYM_COMMA, s 132), tr!(SYM_SEMI, s 133), tr!(SYM_EQUALS, s 134), tr!(SYM_OBRACE, s 135), tr!(SYM_OBRACE, s 136), tr!(SYM_PLUS, s 137), tr!(SYM_OBRACE, s 138), tr!(SYM_EQUALS, s 139), tr!(SYM_EQUALS, s 140), tr!(SYM_OBRACE, s 141), tr!(SYM_EQUALS, s 142), tr!(SYM_OBRACE, s 143), tr!(SYM_OBRACE, s 144),
    tr!(SYM_OBRACE, s 145), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_Integer, g 146), tr!(SYM_SEMI, s 147), tr!(SYM_OBRACE, s 148), tr!(SYM_EQUALS, s 149), tr!(SYM_DECIMAL_DIGIT, s 150), tr!(SYM_INTEGER, s 151), tr!(SYM_OBRACE, s 153), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158),
    tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93),
    tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 170), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70),
    tr!(SYM_FieldSpec, g 177), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166),
    tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 178), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_VModDef, g 179), tr!(SYM_Ident, g 96), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155),
    tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97),
    tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 180), tr!(SYM_VarDeclList, g 182), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160),
    tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70),
    tr!(SYM_Expr, g 183), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_OBRACKET, s 184), tr!(SYM_EXCLAM, s 185), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70),
    tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 190), tr!(SYM_Ident, g 191), tr!(SYM_KEYNAME, s 192), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163),
    tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 193), tr!(SYM_SECTION, s 99), tr!(SYM_STRING, s 100), tr!(SYM_DECIMAL_DIGIT, s 101),
    tr!(SYM_INTEGER, s 102), tr!(SYM_IDENT, s 103), tr!(SYM_KEYNAME, s 194), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164),
    tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 198), tr!(SYM_VarDeclList, g 199), tr!(SYM_OBRACE, s 200), tr!(SYM_OBRACKET, s 201), tr!(SYM_IDENT, s 93),
    tr!(SYM_DEFAULT, s 70), tr!(SYM_Ident, g 206), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 207), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 208), tr!(SYM_SECTION, s 161), tr!(SYM_OVERLAY, s 209), tr!(SYM_TEXT, s 62), tr!(SYM_OUTLINE, s 63), tr!(SYM_SOLID, s 64),
    tr!(SYM_LOGO, s 65), tr!(SYM_EXCLAM, s 67), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_VarDecl, g 210), tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 90), tr!(SYM_Ident, g 91), tr!(SYM_EQUALS, s 216), tr!(SYM_VarDeclList, g 217), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158),
    tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93),
    tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 218), tr!(SYM_SEMI, s 219), tr!(SYM_VarDeclList, g 220), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162),
    tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Term, g 221), tr!(SYM_Lhs, g 222), tr!(SYM_TYPE, s 154),
    tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168),
    tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Term, g 223), tr!(SYM_Lhs, g 222), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159),
    tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_CBRACE, s 224), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_FieldSpec, g 225), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160),
    tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70),
    tr!(SYM_Expr, g 228), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166),
    tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Term, g 229), tr!(SYM_Lhs, g 222), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157),
    tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119),
    tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Term, g 230), tr!(SYM_Lhs, g 222), tr!(SYM_OPAREN, s 231), tr!(SYM_OBRACKET, s 128), tr!(SYM_DOT, s 129), tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_CBRACKET, s 236), tr!(SYM_EQUALS, s 237), tr!(SYM_OBRACKET, s 238), tr!(SYM_PLUS, s 232),
    tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_SEMI, s 239), tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160),
    tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_CBRACE, s 240), tr!(SYM_EXCLAM, s 67), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 90), tr!(SYM_Ident, g 91), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158),
    tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_CBRACE, s 242), tr!(SYM_EXCLAM, s 67), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 90), tr!(SYM_Ident, g 91), tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235),
    tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 243), tr!(SYM_TEXT, s 162), tr!(SYM_OBRACE, s 244), tr!(SYM_STRING, s 245), tr!(SYM_DECIMAL_DIGIT, s 101), tr!(SYM_INTEGER, s 102), tr!(SYM_IDENT, s 246),
    tr!(SYM_DEFAULT, s 70), tr!(SYM_FieldSpec, g 225), tr!(SYM_NonEmptyActions, g 249), tr!(SYM_Action, g 250), tr!(SYM_KeySymLit, g 253), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_Ident, g 254), tr!(SYM_CBRACE, s 255), tr!(SYM_COMMA, s 256), tr!(SYM_EQUALS, s 257), tr!(SYM_CBRACE, r 71), tr!(SYM_COMMA, r 71), tr!(SYM_SEMI, s 258), tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233),
    tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_SEMI, s 259), tr!(SYM_CBRACE, s 260), tr!(SYM_COMMA, s 261), tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_SEMI, s 262), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158),
    tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_CBRACE, s 263), tr!(SYM_EXCLAM, s 67), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 90), tr!(SYM_Ident, g 91), tr!(SYM_OBRACKET, s 201), tr!(SYM_CoordList, g 264), tr!(SYM_MINUS, s 265), tr!(SYM_DECIMAL_DIGIT, s 266),
    tr!(SYM_INTEGER, s 267), tr!(SYM_FLOAT, s 268), tr!(SYM_CBRACE, s 271), tr!(SYM_COMMA, s 272), tr!(SYM_CBRACE, s 273), tr!(SYM_COMMA, s 274), tr!(SYM_EQUALS, s 275), tr!(SYM_STRING, s 97), tr!(SYM_String, g 114), tr!(SYM_OBRACE, s 276), tr!(SYM_STRING, s 97), tr!(SYM_String, g 277), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156),
    tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 207), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 208), tr!(SYM_SECTION, s 161), tr!(SYM_OVERLAY, s 209), tr!(SYM_TEXT, s 62), tr!(SYM_OUTLINE, s 63), tr!(SYM_SOLID, s 64), tr!(SYM_LOGO, s 65), tr!(SYM_CBRACE, s 278), tr!(SYM_EXCLAM, s 67), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_VarDecl, g 210),
    tr!(SYM_SectionBodyItem, g 279), tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 90), tr!(SYM_Ident, g 91), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163),
    tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 280), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53),
    tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_CBRACE, s 281), tr!(SYM_EXCLAM, s 67), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 90), tr!(SYM_Ident, g 91), tr!(SYM_PLUS, s 232),
    tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_SEMI, s 282), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_CBRACE, s 283),
    tr!(SYM_EXCLAM, s 67), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 90), tr!(SYM_Ident, g 91), tr!(SYM_OPAREN, s 284), tr!(SYM_CBRACE, s 285), tr!(SYM_COMMA, s 286), tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_CPAREN, s 287), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155),
    tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97),
    tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161),
    tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 290), tr!(SYM_TYPE, s 154),
    tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168),
    tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 291), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160),
    tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70),
    tr!(SYM_Expr, g 292), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166),
    tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 293), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158),
    tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93),
    tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 294), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164),
    tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 295), tr!(SYM_SEMI, s 296), tr!(SYM_SEMI, s 297), tr!(SYM_OPAREN, r 139), tr!(SYM_TYPE, s 154),
    tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 243), tr!(SYM_TEXT, s 162), tr!(SYM_CBRACE, s 298), tr!(SYM_STRING, s 299), tr!(SYM_DECIMAL_DIGIT, s 101), tr!(SYM_INTEGER, s 102), tr!(SYM_IDENT, s 246), tr!(SYM_DEFAULT, s 70),
    tr!(SYM_FieldSpec, g 225), tr!(SYM_KeySymLit, g 301), tr!(SYM_OPAREN, r 214), tr!(SYM_CBRACKET, s 302), tr!(SYM_COMMA, s 303), tr!(SYM_CBRACKET, s 304), tr!(SYM_COMMA, s 305), tr!(SYM_CBRACKET, s 306), tr!(SYM_COMMA, s 307), tr!(SYM_SEMI, s 308), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158),
    tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_OBRACKET, s 184), tr!(SYM_EXCLAM, s 185), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_SymbolsVarDecl, g 309), tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 190), tr!(SYM_Ident, g 191), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53),
    tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_OBRACKET, s 184), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97),
    tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_MultiKeySymOrActionList, g 310), tr!(SYM_Expr, g 311), tr!(SYM_SEMI, s 312), tr!(SYM_SECTION, s 99), tr!(SYM_STRING, s 100), tr!(SYM_DECIMAL_DIGIT, s 101), tr!(SYM_INTEGER, s 102), tr!(SYM_IDENT, s 103), tr!(SYM_KEYNAME, s 194), tr!(SYM_KeyOrKeySym, g 313),
    tr!(SYM_SEMI, s 314), tr!(SYM_CBRACE, s 315), tr!(SYM_COMMA, s 274), tr!(SYM_DECIMAL_DIGIT, s 266), tr!(SYM_INTEGER, s 267), tr!(SYM_FLOAT, s 268), tr!(SYM_Number, g 316), tr!(SYM_COMMA, s 317), tr!(SYM_SEMI, s 318), tr!(SYM_OBRACE, s 200), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_OutlineInList, g 319), tr!(SYM_Ident, g 206), tr!(SYM_SEMI, s 320), tr!(SYM_OBRACKET, s 201),
    tr!(SYM_Coord, g 321), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 322), tr!(SYM_OPAREN, s 166),
    tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 323), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158),
    tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_KEYS, s 324), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_EXCLAM, s 67), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_VarDecl, g 325), tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 90), tr!(SYM_Ident, g 91), tr!(SYM_OBRACE, s 328), tr!(SYM_SEMI, s 329), tr!(SYM_PLUS, s 232),
    tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_SEMI, s 330), tr!(SYM_SEMI, s 331), tr!(SYM_SEMI, s 332), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161),
    tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_ExprList, g 333), tr!(SYM_TYPE, s 154),
    tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_FieldSpec, g 225), tr!(SYM_Action, g 334), tr!(SYM_CPAREN, s 335), tr!(SYM_COMMA, s 336),
    tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235),
    tr!(SYM_CBRACKET, s 337), tr!(SYM_CBRACE, s 338), tr!(SYM_COMMA, s 339), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 243), tr!(SYM_TEXT, s 162), tr!(SYM_OBRACE, s 340), tr!(SYM_STRING, s 245),
    tr!(SYM_DECIMAL_DIGIT, s 101), tr!(SYM_INTEGER, s 102), tr!(SYM_IDENT, s 246), tr!(SYM_DEFAULT, s 70), tr!(SYM_FieldSpec, g 225), tr!(SYM_MultiActionList, g 341), tr!(SYM_NonEmptyActions, g 249), tr!(SYM_Action, g 250), tr!(SYM_MultiKeySymList, g 342), tr!(SYM_KeySymLit, g 253), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158),
    tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_OBRACE, s 165), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_FieldSpec, g 225), tr!(SYM_Actions, g 343), tr!(SYM_Action, g 344), tr!(SYM_SECTION, s 99), tr!(SYM_OBRACE, s 345), tr!(SYM_STRING, s 245), tr!(SYM_DECIMAL_DIGIT, s 101), tr!(SYM_INTEGER, s 102),
    tr!(SYM_IDENT, s 103), tr!(SYM_NonEmptyKeySyms, g 346), tr!(SYM_KeySymLit, g 348), tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_MINUS, s 265), tr!(SYM_DECIMAL_DIGIT, s 266), tr!(SYM_INTEGER, s 267), tr!(SYM_FLOAT, s 268), tr!(SYM_SignedNumber, g 349), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156),
    tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_CBRACE, s 224), tr!(SYM_OBRACKET, s 201), tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_CoordList, g 350), tr!(SYM_FieldSpec, g 225), tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234),
    tr!(SYM_TIMES, s 235), tr!(SYM_OBRACE, s 351), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_KEYS, s 324), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_CBRACE, s 352), tr!(SYM_EXCLAM, s 67),
    tr!(SYM_IDENT, s 93), tr!(SYM_DEFAULT, s 70), tr!(SYM_VarDecl, g 325), tr!(SYM_RowBodyItem, g 353), tr!(SYM_FieldSpec, g 88), tr!(SYM_Lhs, g 90), tr!(SYM_Ident, g 91), tr!(SYM_KEYNAME, s 354), tr!(SYM_CPAREN, s 357), tr!(SYM_COMMA, s 336), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158),
    tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163), tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93),
    tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_Expr, g 358), tr!(SYM_SECTION, s 99), tr!(SYM_STRING, s 359), tr!(SYM_DECIMAL_DIGIT, s 101), tr!(SYM_INTEGER, s 102), tr!(SYM_IDENT, s 103), tr!(SYM_KeySymLit, g 360), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159),
    tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 243), tr!(SYM_TEXT, s 162), tr!(SYM_CBRACE, s 361), tr!(SYM_STRING, s 299), tr!(SYM_DECIMAL_DIGIT, s 101), tr!(SYM_INTEGER, s 102), tr!(SYM_IDENT, s 246), tr!(SYM_DEFAULT, s 70), tr!(SYM_FieldSpec, g 225), tr!(SYM_KeySymLit, g 301), tr!(SYM_CBRACKET, s 362), tr!(SYM_COMMA, s 305), tr!(SYM_CBRACKET, s 363), tr!(SYM_COMMA, s 307),
    tr!(SYM_SECTION, s 99), tr!(SYM_CBRACE, s 364), tr!(SYM_STRING, s 299), tr!(SYM_DECIMAL_DIGIT, s 101), tr!(SYM_INTEGER, s 102), tr!(SYM_IDENT, s 103), tr!(SYM_KeySymLit, g 301), tr!(SYM_CBRACKET, s 365), tr!(SYM_CBRACE, s 366), tr!(SYM_COMMA, s 274), tr!(SYM_OBRACE, s 367), tr!(SYM_KEYNAME, s 368), tr!(SYM_SEMI, s 371), tr!(SYM_EQUALS, s 372), tr!(SYM_CBRACE, s 373), tr!(SYM_COMMA, s 374),
    tr!(SYM_PLUS, s 232), tr!(SYM_MINUS, s 233), tr!(SYM_DIVIDE, s 234), tr!(SYM_TIMES, s 235), tr!(SYM_TYPE, s 154), tr!(SYM_INTERPRET, s 155), tr!(SYM_ACTION_TOK, s 53), tr!(SYM_KEY, s 156), tr!(SYM_GROUP, s 157), tr!(SYM_MODIFIER_MAP, s 158), tr!(SYM_INDICATOR, s 159), tr!(SYM_SHAPE, s 160), tr!(SYM_ROW, s 60), tr!(SYM_SECTION, s 161), tr!(SYM_TEXT, s 162), tr!(SYM_PLUS, s 163),
    tr!(SYM_MINUS, s 164), tr!(SYM_OBRACE, s 165), tr!(SYM_OPAREN, s 166), tr!(SYM_EXCLAM, s 167), tr!(SYM_INVERT, s 168), tr!(SYM_STRING, s 97), tr!(SYM_DECIMAL_DIGIT, s 109), tr!(SYM_INTEGER, s 110), tr!(SYM_FLOAT, s 119), tr!(SYM_IDENT, s 93), tr!(SYM_KEYNAME, s 120), tr!(SYM_DEFAULT, s 70), tr!(SYM_ExprList, g 375), tr!(SYM_CBRACE, s 376), tr!(SYM_COMMA, s 377), tr!(SYM_KEYNAME, s 378),
    tr!(SYM_SEMI, s 379), tr!(SYM_KEYNAME, s 354), tr!(SYM_OverlayKey, g 380), tr!(SYM_CBRACE, s 381), tr!(SYM_COMMA, s 336), tr!(SYM_SEMI, s 382), tr!(SYM_OBRACE, s 367), tr!(SYM_KEYNAME, s 368), tr!(SYM_Key, g 383),
];

#[rustfmt::skip]
pub(crate) static STATES: &[State; 384] = &[
    st!(0, 9, r 18), st!(9, 0, r 4), st!(9, 0, r 21), st!(9, 0, r 22), st!(9, 0, r 23), st!(9, 0, r 24), st!(9, 0, r 25), st!(9, 0, r 26),
    st!(9, 0, r 27), st!(9, 0, r 28), st!(9, 1, e), st!(10, 0, r 2), st!(10, 0, r 3), st!(10, 8, e), st!(18, 9, r 17), st!(27, 0, r 20),
    st!(27, 0, a), st!(27, 0, r 6), st!(27, 0, r 12), st!(27, 0, r 13), st!(27, 0, r 15), st!(27, 0, r 14), st!(27, 0, r 16), st!(27, 0, r 7),
    st!(27, 0, r 8), st!(27, 1, r 218), st!(28, 2, r 218), st!(30, 0, r 19), st!(30, 0, r 219), st!(30, 1, e), st!(31, 0, r 217), st!(31, 1, e),
    st!(32, 0, r 10), st!(32, 0, r 31), st!(32, 11, r 18), st!(43, 6, r 142), st!(49, 1, e), st!(50, 0, r 9), st!(50, 5, e), st!(55, 0, r 143),
    st!(55, 0, r 145), st!(55, 0, r 144), st!(55, 0, r 146), st!(55, 0, r 147), st!(55, 1, e), st!(56, 0, r 29), st!(56, 27, e), st!(83, 1, r 141),
    st!(84, 0, r 5), st!(84, 0, r 11), st!(84, 3, e), st!(87, 2, r 132), st!(89, 6, r 131), st!(95, 0, r 130), st!(95, 1, r 133), st!(96, 1, e),
    st!(97, 3, r 134), st!(100, 3, r 135), st!(103, 5, r 136), st!(108, 2, r 137), st!(110, 0, r 138), st!(110, 2, r 139), st!(112, 1, r 140), st!(113, 0, r 125),
    st!(113, 0, r 126), st!(113, 0, r 127), st!(113, 1, e), st!(114, 3, e), st!(117, 7, r 214), st!(124, 1, e), st!(125, 0, r 215), st!(125, 0, r 32),
    st!(125, 0, r 34), st!(125, 0, r 35), st!(125, 0, r 30), st!(125, 0, r 33), st!(125, 0, r 36), st!(125, 0, r 37), st!(125, 0, r 39), st!(125, 0, r 38),
    st!(125, 0, r 40), st!(125, 0, r 41), st!(125, 0, r 45), st!(125, 0, r 46), st!(125, 0, r 42), st!(125, 0, r 43), st!(125, 0, r 44), st!(125, 2, e),
    st!(127, 2, r 176), st!(129, 0, r 129), st!(129, 1, e), st!(130, 1, r 128), st!(131, 0, r 47), st!(131, 0, r 214), st!(131, 2, e), st!(133, 0, r 55),
    st!(133, 1, r 56), st!(134, 0, r 216), st!(134, 1, e), st!(135, 0, r 201), st!(135, 0, r 199), st!(135, 0, r 202), st!(135, 0, r 203), st!(135, 0, r 200),
    st!(135, 1, e), st!(136, 1, r 60), st!(137, 0, r 198), st!(137, 1, e), st!(138, 1, e), st!(139, 0, r 211), st!(139, 0, r 210), st!(139, 1, e),
    st!(140, 1, e), st!(141, 1, e), st!(142, 1, e), st!(143, 1, e), st!(144, 1, e), st!(145, 3, e), st!(148, 1, e), st!(149, 0, r 209),
    st!(149, 0, r 185), st!(149, 1, e), st!(150, 1, r 180), st!(151, 0, r 184), st!(151, 0, r 183), st!(151, 0, r 182), st!(151, 2, e), st!(153, 1, e),
    st!(154, 25, e), st!(179, 14, e), st!(193, 25, e), st!(218, 0, r 49), st!(218, 4, e), st!(222, 0, r 53), st!(222, 25, e), st!(247, 0, r 62),
    st!(247, 1, r 62), st!(248, 25, e), st!(273, 18, r 66), st!(291, 1, e), st!(292, 25, e), st!(317, 6, e), st!(323, 25, e), st!(348, 1, r 62),
    st!(349, 5, e), st!(354, 22, e), st!(376, 1, e), st!(377, 0, r 50), st!(377, 1, r 62), st!(378, 25, e), st!(403, 0, r 213), st!(403, 0, r 212),
    st!(403, 1, e), st!(404, 1, r 62), st!(405, 0, r 132), st!(405, 0, r 131), st!(405, 0, r 133), st!(405, 0, r 134), st!(405, 0, r 135), st!(405, 0, r 136),
    st!(405, 0, r 137), st!(405, 0, r 139), st!(405, 0, r 140), st!(405, 26, e), st!(431, 26, e), st!(457, 15, e), st!(472, 25, e), st!(497, 26, e),
    st!(523, 26, e), st!(549, 3, r 176), st!(552, 5, e), st!(557, 0, r 156), st!(557, 0, r 173), st!(557, 0, r 163), st!(557, 1, r 161), st!(558, 0, r 164),
    st!(558, 0, r 128), st!(558, 1, r 177), st!(559, 5, e), st!(564, 0, r 54), st!(564, 4, r 57), st!(568, 18, e), st!(586, 18, e), st!(604, 4, r 59),
    st!(608, 21, r 81), st!(629, 3, e), st!(632, 1, e), st!(633, 1, r 65), st!(634, 0, r 68), st!(634, 0, r 73), st!(634, 1, e), st!(635, 2, r 128),
    st!(637, 1, e), st!(638, 5, e), st!(643, 0, r 86), st!(643, 2, e), st!(645, 0, r 85), st!(645, 0, r 87), st!(645, 5, e), st!(650, 18, e),
    st!(668, 2, e), st!(670, 4, e), st!(674, 2, e), st!(676, 0, r 116), st!(676, 2, e), st!(678, 0, r 121), st!(678, 1, e), st!(679, 2, r 136),
    st!(681, 1, r 138), st!(682, 2, e), st!(684, 0, r 99), st!(684, 0, r 101), st!(684, 24, e), st!(708, 0, r 97), st!(708, 0, r 102), st!(708, 0, r 100),
    st!(708, 25, e), st!(733, 18, e), st!(751, 5, e), st!(756, 0, r 51), st!(756, 18, e), st!(774, 0, r 158), st!(774, 0, r 161), st!(774, 0, r 157),
    st!(774, 0, r 174), st!(774, 1, e), st!(775, 2, e), st!(777, 0, r 171), st!(777, 5, e), st!(782, 0, r 159), st!(782, 0, r 160), st!(782, 24, r 150),
    st!(806, 25, e), st!(831, 25, e), st!(856, 25, e), st!(881, 25, e), st!(906, 0, r 178), st!(906, 25, e), st!(931, 25, e), st!(956, 0, r 48),
    st!(956, 1, e), st!(957, 0, r 61), st!(957, 1, e), st!(958, 1, r 201), st!(959, 19, e), st!(978, 0, r 195), st!(978, 1, r 200), st!(979, 2, e),
    st!(981, 2, e), st!(983, 0, r 169), st!(983, 0, r 168), st!(983, 2, e), st!(985, 0, r 189), st!(985, 0, r 188), st!(985, 0, r 72), st!(985, 1, e),
    st!(986, 19, e), st!(1005, 27, e), st!(1032, 0, r 52), st!(1032, 0, r 82), st!(1032, 1, e), st!(1033, 7, e), st!(1040, 0, r 89), st!(1040, 1, e),
    st!(1041, 2, e), st!(1043, 4, e), st!(1047, 0, r 207), st!(1047, 0, r 208), st!(1047, 0, r 206), st!(1047, 1, e), st!(1048, 0, r 205), st!(1048, 1, e),
    st!(1049, 5, e), st!(1054, 1, e), st!(1055, 2, e), st!(1057, 25, e), st!(1082, 19, e), st!(1101, 1, e), st!(1102, 1, e), st!(1103, 0, r 96),
    st!(1103, 5, e), st!(1108, 1, e), st!(1109, 0, r 91), st!(1109, 1, e), st!(1110, 25, r 150), st!(1135, 0, r 172), st!(1135, 15, e), st!(1150, 0, r 165),
    st!(1150, 2, e), st!(1152, 4, r 149), st!(1156, 2, r 152), st!(1158, 2, r 153), st!(1160, 0, r 151), st!(1160, 0, r 154), st!(1160, 4, r 155), st!(1164, 5, e),
    st!(1169, 0, r 63), st!(1169, 0, r 58), st!(1169, 0, r 80), st!(1169, 0, r 193), st!(1169, 2, e), st!(1171, 0, r 192), st!(1171, 0, r 78), st!(1171, 23, e),
    st!(1194, 0, r 76), st!(1194, 17, e), st!(1211, 0, r 74), st!(1211, 8, e), st!(1219, 0, r 64), st!(1219, 0, r 67), st!(1219, 0, r 70), st!(1219, 4, r 69),
    st!(1223, 0, r 83), st!(1223, 0, r 84), st!(1223, 0, r 88), st!(1223, 0, r 117), st!(1223, 0, r 204), st!(1223, 5, e), st!(1228, 0, r 93), st!(1228, 0, r 115),
    st!(1228, 0, r 94), st!(1228, 0, r 120), st!(1228, 17, e), st!(1245, 4, r 119), st!(1249, 1, e), st!(1250, 0, r 106), st!(1250, 21, e), st!(1271, 0, r 104),
    st!(1271, 1, e), st!(1272, 0, r 95), st!(1272, 0, r 90), st!(1272, 0, r 92), st!(1272, 0, r 123), st!(1272, 2, e), st!(1274, 0, r 170), st!(1274, 0, r 162),
    st!(1274, 25, e), st!(1299, 0, r 179), st!(1299, 0, r 194), st!(1299, 6, e), st!(1305, 19, e), st!(1324, 2, e), st!(1326, 2, e), st!(1328, 0, r 167),
    st!(1328, 0, r 166), st!(1328, 7, e), st!(1335, 0, r 196), st!(1335, 0, r 187), st!(1335, 0, r 186), st!(1335, 1, e), st!(1336, 2, e), st!(1338, 2, e),
    st!(1340, 1, e), st!(1341, 0, r 103), st!(1341, 1, e), st!(1342, 2, e), st!(1344, 0, r 113), st!(1344, 0, r 175), st!(1344, 4, r 148), st!(1348, 0, r 191),
    st!(1348, 0, r 190), st!(1348, 0, r 79), st!(1348, 0, r 77), st!(1348, 0, r 75), st!(1348, 0, r 197), st!(1348, 0, r 122), st!(1348, 0, r 118), st!(1348, 25, r 150),
    st!(1373, 0, r 109), st!(1373, 2, e), st!(1375, 0, r 108), st!(1375, 0, r 98), st!(1375, 1, e), st!(1376, 1, e), st!(1377, 2, e), st!(1379, 2, e),
    st!(1381, 1, e), st!(1382, 3, e), st!(1385, 0, r 114), st!(1385, 0, r 111), st!(1385, 0, r 112), st!(1385, 0, r 110), st!(1385, 0, r 105), st!(1385, 0, r 107),
];

#[rustfmt::skip]
pub(crate) static RULES: &[Rule; 220] = &[
    Rule(SYM_EOF, 0, 0), Rule(SYM_accept, 2, 0), Rule(SYM_XkbFile, 1, 10), Rule(SYM_XkbFile, 1, 10), Rule(SYM_XkbFile, 1, 10), Rule(SYM_XkbCompositeMap, 7, 11), Rule(SYM_XkbCompositeType, 1, 25), Rule(SYM_XkbCompositeType, 1, 25), Rule(SYM_XkbCompositeType, 1, 25), Rule(SYM_XkbMapConfigList, 2, 34), Rule(SYM_XkbMapConfigList, 0, 34), Rule(SYM_XkbMapConfig, 7, 12),
    Rule(SYM_FileType, 1, 26), Rule(SYM_FileType, 1, 26), Rule(SYM_FileType, 1, 26), Rule(SYM_FileType, 1, 26), Rule(SYM_FileType, 1, 26), Rule(SYM_OptFlags, 1, 13), Rule(SYM_OptFlags, 0, 13), Rule(SYM_Flags, 2, 14), Rule(SYM_Flags, 1, 14), Rule(SYM_Flag, 1, 15), Rule(SYM_Flag, 1, 15), Rule(SYM_Flag, 1, 15),
    Rule(SYM_Flag, 1, 15), Rule(SYM_Flag, 1, 15), Rule(SYM_Flag, 1, 15), Rule(SYM_Flag, 1, 15), Rule(SYM_Flag, 1, 15), Rule(SYM_DeclList, 2, 35), Rule(SYM_DeclList, 3, 35), Rule(SYM_DeclList, 0, 35), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45),
    Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45), Rule(SYM_Decl, 2, 45),
    Rule(SYM_VarDecl, 4, 241), Rule(SYM_VarDecl, 2, 241), Rule(SYM_VarDecl, 3, 241), Rule(SYM_KeyNameDecl, 4, 72), Rule(SYM_KeyAliasDecl, 5, 73), Rule(SYM_VModDecl, 3, 74), Rule(SYM_VModDefList, 3, 94), Rule(SYM_VModDefList, 1, 94), Rule(SYM_VModDef, 1, 95), Rule(SYM_VModDef, 3, 95), Rule(SYM_InterpretDecl, 6, 75), Rule(SYM_InterpretMatch, 3, 104),
    Rule(SYM_InterpretMatch, 1, 104), Rule(SYM_VarDeclList, 2, 181), Rule(SYM_VarDeclList, 0, 181), Rule(SYM_KeyTypeDecl, 6, 76), Rule(SYM_SymbolsDecl, 6, 77), Rule(SYM_OptSymbolsBody, 1, 186), Rule(SYM_OptSymbolsBody, 0, 186), Rule(SYM_SymbolsBody, 3, 187), Rule(SYM_SymbolsBody, 1, 187), Rule(SYM_SymbolsVarDecl, 3, 188), Rule(SYM_SymbolsVarDecl, 3, 188), Rule(SYM_SymbolsVarDecl, 1, 188),
    Rule(SYM_SymbolsVarDecl, 2, 188), Rule(SYM_SymbolsVarDecl, 1, 188), Rule(SYM_MultiKeySymOrActionList, 3, 189), Rule(SYM_MultiKeySymOrActionList, 5, 189), Rule(SYM_MultiKeySymOrActionList, 3, 189), Rule(SYM_MultiKeySymOrActionList, 5, 189), Rule(SYM_MultiKeySymOrActionList, 3, 189), Rule(SYM_NoSymbolOrActionList, 4, 247), Rule(SYM_NoSymbolOrActionList, 2, 247), Rule(SYM_NoSymbolOrActionList, 0, 247), Rule(SYM_GroupCompatDecl, 5, 78), Rule(SYM_ModMapDecl, 6, 79),
    Rule(SYM_KeyOrKeySymList, 3, 195), Rule(SYM_KeyOrKeySymList, 1, 195), Rule(SYM_KeyOrKeySym, 1, 196), Rule(SYM_KeyOrKeySym, 1, 196), Rule(SYM_LedMapDecl, 6, 211), Rule(SYM_LedNameDecl, 5, 81), Rule(SYM_LedNameDecl, 6, 81), Rule(SYM_UnknownDecl, 5, 82), Rule(SYM_UnknownCompoundStatementDecl, 6, 83), Rule(SYM_ShapeDecl, 6, 84), Rule(SYM_ShapeDecl, 6, 84), Rule(SYM_SectionDecl, 6, 85),
    Rule(SYM_SectionBody, 2, 212), Rule(SYM_SectionBody, 1, 212), Rule(SYM_SectionBodyItem, 5, 213), Rule(SYM_SectionBodyItem, 1, 213), Rule(SYM_SectionBodyItem, 1, 213), Rule(SYM_SectionBodyItem, 1, 213), Rule(SYM_SectionBodyItem, 1, 213), Rule(SYM_RowBody, 2, 326), Rule(SYM_RowBody, 1, 326), Rule(SYM_RowBodyItem, 5, 327), Rule(SYM_RowBodyItem, 1, 327), Rule(SYM_Keys, 3, 369),
    Rule(SYM_Keys, 1, 369), Rule(SYM_Key, 1, 370), Rule(SYM_Key, 3, 370), Rule(SYM_OverlayDecl, 6, 214), Rule(SYM_OverlayKeyList, 3, 355), Rule(SYM_OverlayKeyList, 1, 355), Rule(SYM_OverlayKey, 3, 356), Rule(SYM_OutlineList, 3, 202), Rule(SYM_OutlineList, 1, 202), Rule(SYM_OutlineInList, 3, 203), Rule(SYM_OutlineInList, 5, 203), Rule(SYM_OutlineInList, 3, 203),
    Rule(SYM_CoordList, 3, 204), Rule(SYM_CoordList, 1, 204), Rule(SYM_Coord, 5, 205), Rule(SYM_DoodadDecl, 6, 215), Rule(SYM_DoodadType, 1, 87), Rule(SYM_DoodadType, 1, 87), Rule(SYM_DoodadType, 1, 87), Rule(SYM_DoodadType, 1, 87), Rule(SYM_FieldSpec, 1, 169), Rule(SYM_FieldSpec, 1, 169), Rule(SYM_Element, 1, 89), Rule(SYM_Element, 1, 89),
    Rule(SYM_Element, 1, 89), Rule(SYM_Element, 1, 89), Rule(SYM_Element, 1, 89), Rule(SYM_Element, 1, 89), Rule(SYM_Element, 1, 89), Rule(SYM_Element, 1, 89), Rule(SYM_Element, 1, 89), Rule(SYM_Element, 1, 89), Rule(SYM_Element, 1, 89), Rule(SYM_OptMergeMode, 1, 46), Rule(SYM_OptMergeMode, 0, 46), Rule(SYM_MergeMode, 1, 47),
    Rule(SYM_MergeMode, 1, 47), Rule(SYM_MergeMode, 1, 47), Rule(SYM_MergeMode, 1, 47), Rule(SYM_MergeMode, 1, 47), Rule(SYM_ExprList, 3, 288), Rule(SYM_ExprList, 1, 288), Rule(SYM_ExprList, 0, 288), Rule(SYM_Expr, 3, 289), Rule(SYM_Expr, 3, 289), Rule(SYM_Expr, 3, 289), Rule(SYM_Expr, 3, 289), Rule(SYM_Expr, 3, 289),
    Rule(SYM_Expr, 1, 289), Rule(SYM_Term, 2, 171), Rule(SYM_Term, 2, 171), Rule(SYM_Term, 2, 171), Rule(SYM_Term, 2, 171), Rule(SYM_Term, 1, 171), Rule(SYM_Term, 4, 171), Rule(SYM_Term, 1, 171), Rule(SYM_Term, 1, 171), Rule(SYM_Term, 3, 171), Rule(SYM_MultiActionList, 3, 248), Rule(SYM_MultiActionList, 3, 248),
    Rule(SYM_MultiActionList, 1, 248), Rule(SYM_MultiActionList, 1, 248), Rule(SYM_ActionList, 3, 226), Rule(SYM_ActionList, 1, 226), Rule(SYM_NonEmptyActions, 3, 172), Rule(SYM_Actions, 1, 173), Rule(SYM_Actions, 2, 173), Rule(SYM_Action, 4, 227), Rule(SYM_Lhs, 1, 174), Rule(SYM_Lhs, 3, 174), Rule(SYM_Lhs, 4, 174), Rule(SYM_Lhs, 6, 174),
    Rule(SYM_OptTerminal, 1, 121), Rule(SYM_OptTerminal, 0, 121), Rule(SYM_Terminal, 1, 175), Rule(SYM_Terminal, 1, 175), Rule(SYM_Terminal, 1, 175), Rule(SYM_Terminal, 1, 175), Rule(SYM_MultiKeySymList, 3, 251), Rule(SYM_MultiKeySymList, 3, 251), Rule(SYM_MultiKeySymList, 1, 251), Rule(SYM_MultiKeySymList, 1, 251), Rule(SYM_KeySymList, 3, 300), Rule(SYM_KeySymList, 3, 300),
    Rule(SYM_KeySymList, 1, 300), Rule(SYM_KeySymList, 1, 300), Rule(SYM_NonEmptyKeySyms, 3, 252), Rule(SYM_NonEmptyKeySyms, 1, 252), Rule(SYM_KeySyms, 1, 347), Rule(SYM_KeySyms, 2, 347), Rule(SYM_KeySym, 1, 197), Rule(SYM_KeySym, 1, 197), Rule(SYM_KeySymLit, 1, 106), Rule(SYM_KeySymLit, 1, 106), Rule(SYM_KeySymLit, 1, 106), Rule(SYM_KeySymLit, 1, 106),
    Rule(SYM_SignedNumber, 2, 269), Rule(SYM_SignedNumber, 1, 269), Rule(SYM_Number, 1, 270), Rule(SYM_Number, 1, 270), Rule(SYM_Number, 1, 270), Rule(SYM_Float, 1, 123), Rule(SYM_Integer, 1, 124), Rule(SYM_Integer, 1, 124), Rule(SYM_KeyCode, 1, 152), Rule(SYM_KeyCode, 1, 152), Rule(SYM_Ident, 1, 176), Rule(SYM_Ident, 1, 176),
    Rule(SYM_String, 1, 125), Rule(SYM_OptMapName, 1, 29), Rule(SYM_OptMapName, 0, 29), Rule(SYM_MapName, 1, 30),
];
