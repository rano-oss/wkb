use super::keysym::xkb_keysym_from_name;
use super::parser::*;
use crate::xkb::keysym::codepoint_to_keysym;
use std::sync::Arc;
pub(crate) fn braced_end(input: &[u8], mut pos: usize) -> Option<usize> {
    let mut depth = 1;
    let mut quoted = false;
    while pos < input.len() {
        let byte = input[pos];
        if quoted {
            if byte == b'\\' {
                pos += 1;
            } else if byte == b'"' {
                quoted = false;
            }
        } else if byte == b'"' {
            quoted = true;
        } else if byte == b'#' || input[pos..].starts_with(b"//") {
            pos += input[pos..]
                .iter()
                .position(|&byte| byte == b'\n')
                .unwrap_or(input.len() - pos);
            continue;
        } else if byte == b'{' {
            depth += 1;
        } else if byte == b'}' {
            depth -= 1;
            if depth == 0 {
                return Some(pos);
            }
        }
        pos += 1;
    }
    None
}
#[derive(Default)]
enum Token<'a> {
    Word(&'a [u8]),
    String(&'a [u8]),
    Key(&'a [u8]),
    Integer(i64),
    Punct(u8),
    #[default]
    End,
    Error,
}
struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
}
impl<'a> Lexer<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, pos: 0 }
    }
    fn take_while(&mut self, predicate: impl Fn(u8) -> bool) -> &'a [u8] {
        let start = self.pos;
        while self.input.get(self.pos).copied().is_some_and(&predicate) {
            self.pos += 1;
        }
        &self.input[start..self.pos]
    }
    fn next(&mut self) -> Token<'a> {
        loop {
            self.take_while(|byte| byte.is_ascii_whitespace());
            if self.input[self.pos..].starts_with(&[0xe2, 0x80, 0x8e])
                || self.input[self.pos..].starts_with(&[0xe2, 0x80, 0x8f])
            {
                self.pos += 3;
                continue;
            }
            if self.input[self.pos..].starts_with(b"//") || self.input.get(self.pos) == Some(&b'#')
            {
                self.pos += self.input[self.pos..]
                    .iter()
                    .position(|&byte| byte == b'\n')
                    .unwrap_or(self.input.len() - self.pos);
                continue;
            }
            break;
        }
        let Some(&byte) = self.input.get(self.pos) else {
            return Token::End;
        };
        if byte == b'"' {
            self.pos += 1;
            let start = self.pos;
            while let Some(&next) = self.input.get(self.pos) {
                if next == b'"' {
                    let value = &self.input[start..self.pos];
                    self.pos += 1;
                    return Token::String(value);
                }
                if next == b'\n' {
                    return Token::Error;
                }
                self.pos += 1 + usize::from(next == b'\\');
            }
            return Token::Error;
        }
        if byte == b'<' {
            self.pos += 1;
            let value = self.take_while(|byte| byte.is_ascii_graphic() && byte != b'>');
            if self.input.get(self.pos) != Some(&b'>') {
                return Token::Error;
            }
            self.pos += 1;
            return Token::Key(value);
        }
        if b";{}=[]().,+-*/!~".contains(&byte) {
            self.pos += 1;
            return Token::Punct(byte);
        }
        if byte.is_ascii_alphabetic() || byte == b'_' {
            return Token::Word(self.take_while(|b| b.is_ascii_alphanumeric() || b == b'_'));
        }
        if !byte.is_ascii() {
            let text = std::str::from_utf8(&self.input[self.pos..]).ok();
            let len = text
                .and_then(|text| text.chars().next())
                .map(char::len_utf8);
            let len = match len {
                Some(len) => len,
                None => return Token::Error,
            };
            let start = self.pos;
            self.pos += len;
            return Token::Word(&self.input[start..self.pos]);
        }
        if byte.is_ascii_digit() {
            let start = self.pos;
            let hex =
                self.input[start..].starts_with(b"0x") || self.input[start..].starts_with(b"0X");
            self.pos += usize::from(hex) * 2;
            self.take_while(|byte| {
                if hex {
                    byte.is_ascii_hexdigit()
                } else {
                    byte.is_ascii_digit()
                }
            });
            let digits = if hex {
                &self.input[start + 2..self.pos]
            } else {
                &self.input[start..self.pos]
            };
            return std::str::from_utf8(digits)
                .ok()
                .and_then(|s| i64::from_str_radix(s, if hex { 16 } else { 10 }).ok())
                .map(Token::Integer)
                .unwrap_or(Token::Error);
        }
        self.pos += 1;
        Token::Error
    }
}
struct Parser<'a> {
    lexer: Lexer<'a>,
    token: Token<'a>,
}
struct MapSpan {
    name: String,
    file_type: FileType,
    flags: u32,
    body: std::ops::Range<usize>,
}
pub(crate) struct SelectedMap<'a> {
    pub(crate) file_type: FileType,
    pub(crate) body: &'a [u8],
}
pub(crate) struct OwnedMap {
    data: Arc<Vec<u8>>,
    body: std::ops::Range<usize>,
    pub(crate) file_type: FileType,
    pub(crate) flags: u32,
}

impl OwnedMap {
    pub(crate) fn stream(&self) -> Stream<'_> {
        Stream::new(&self.data[self.body.clone()])
    }
}
pub(crate) struct Stream<'a> {
    parser: Parser<'a>,
}
impl<'a> Stream<'a> {
    pub(crate) fn new(input: &'a [u8]) -> Self {
        Self {
            parser: Parser::new(input),
        }
    }
    pub(crate) fn next_statement(
        &mut self,
        ctx: &mut XkbContext,
    ) -> Result<Option<Statement<'a>>, ()> {
        loop {
            if matches!(self.parser.token, Token::End) {
                return Ok(None);
            }
            if matches!(self.parser.token, Token::Error) {
                return Err(());
            }
            let pos = self.parser.lexer.pos;
            if let Some(statement) = self.parser.parse_statement(ctx) {
                return Ok(Some(statement));
            }
            if self.parser.lexer.pos == pos {
                return Err(());
            }
        }
    }
    pub(crate) fn next_var(&mut self, ctx: &mut XkbContext) -> Result<Option<VarDef>, ()> {
        if matches!(self.parser.token, Token::End) {
            return Ok(None);
        }
        if matches!(self.parser.token, Token::Error) {
            return Err(());
        }
        self.parser.parse_body_var(ctx).map(Some).ok_or(())
    }
    pub(crate) fn next_map(&mut self) -> Result<Option<SelectedMap<'a>>, ()> {
        if matches!(self.parser.token, Token::End) {
            return Ok(None);
        }
        if matches!(self.parser.token, Token::Error) {
            return Err(());
        }
        let span = self.parser.scan_file().ok_or(())?;
        Ok(Some(SelectedMap {
            file_type: span.file_type,
            body: &self.parser.lexer.input[span.body],
        }))
    }
}
impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        let mut lexer = Lexer::new(input);
        let token = lexer.next();
        Self { lexer, token }
    }
    fn bump(&mut self) -> Token<'a> {
        let next = self.lexer.next();
        std::mem::replace(&mut self.token, next)
    }
    fn punct(&mut self, byte: u8) -> bool {
        if matches!(self.token, Token::Punct(found) if found == byte) {
            self.bump();
            true
        } else {
            false
        }
    }
    fn word(&self, expected: &[u8]) -> bool {
        matches!(self.token, Token::Word(word) if word.eq_ignore_ascii_case(expected))
    }
    fn take_word(&mut self) -> Option<&'a [u8]> {
        match self.bump() {
            Token::Word(word) => Some(word),
            _ => None,
        }
    }
    fn atom(ctx: &mut XkbContext, word: &[u8]) -> u32 {
        ctx.atom_intern(word)
    }
    fn file_type(&mut self) -> Option<FileType> {
        Some(match self.take_word()? {
            word if word.eq_ignore_ascii_case(b"xkb_keymap")
                || word.eq_ignore_ascii_case(b"xkb_layout")
                || word.eq_ignore_ascii_case(b"xkb_semantics") =>
            {
                FileType::Keymap
            }
            word if word.eq_ignore_ascii_case(b"xkb_keycodes") => FileType::Keycodes,
            word if word.eq_ignore_ascii_case(b"xkb_types") => FileType::Types,
            word if word.eq_ignore_ascii_case(b"xkb_symbols") => FileType::Symbols,
            word if word.eq_ignore_ascii_case(b"xkb_compat")
                || word.eq_ignore_ascii_case(b"xkb_compat_map")
                || word.eq_ignore_ascii_case(b"xkb_compatibility")
                || word.eq_ignore_ascii_case(b"xkb_compatibility_map")
                || word.eq_ignore_ascii_case(b"xkb_geometry") =>
            {
                FileType::Ignored
            }
            _ => return None,
        })
    }
    fn map_flags(&mut self) -> u32 {
        let mut flags = 0;
        while matches!(self.token, Token::Word(_)) {
            if self.word(b"default") {
                flags |= MAP_IS_DEFAULT;
            } else if [
                b"partial".as_slice(),
                b"hidden",
                b"alphanumeric_keys",
                b"modifier_keys",
                b"keypad_keys",
                b"function_keys",
                b"alternate_group",
            ]
            .iter()
            .any(|flag| self.word(flag))
            {
                flags |= 2;
            } else {
                break;
            }
            self.bump();
        }
        flags
    }
    fn scan_file(&mut self) -> Option<MapSpan> {
        let flags = self.map_flags();
        let file_type = self.file_type()?;
        let name = match self.token {
            Token::String(_) => match self.bump() {
                Token::String(name) => String::from_utf8_lossy(name).into_owned(),
                _ => unreachable!(),
            },
            _ => String::new(),
        };
        matches!(self.token, Token::Punct(b'{')).then_some(())?;
        let body_start = self.lexer.pos;
        let body_end = braced_end(self.lexer.input, body_start)?;
        self.lexer.pos = body_end + 1;
        self.token = self.lexer.next();
        self.punct(b';');
        Some(MapSpan {
            name,
            file_type,
            flags,
            body: body_start..body_end,
        })
    }
    fn merge(&mut self) -> MergeMode {
        let merge = if self.word(b"augment") {
            MergeMode::Augment
        } else if self.word(b"override") {
            MergeMode::Override
        } else if self.word(b"replace") {
            MergeMode::Replace
        } else {
            return MergeMode::Default;
        };
        self.bump();
        merge
    }
    fn parse_statement(&mut self, ctx: &mut XkbContext) -> Option<Statement<'a>> {
        let merge = self.merge();
        if self.word(b"include") {
            self.bump();
            let Token::String(value) = self.bump() else {
                return None;
            };
            self.punct(b';');
            return std::str::from_utf8(value)
                .ok()
                .and_then(|value| include_create(value, merge))
                .map(Statement::Include);
        }
        if self.word(b"virtual_modifiers") {
            self.bump();
            let mut values = Vec::new();
            loop {
                let name_word = self.take_word()?;
                let name = Self::atom(ctx, name_word);
                let value = self.punct(b'=').then(|| self.parse_expr(ctx, 0)).flatten();
                values.push(VModDef { merge, name, value });
                if !self.punct(b',') {
                    break;
                }
            }
            self.punct(b';').then_some(())?;
            return Some(Statement::VMods(values));
        }
        if self.word(b"type") {
            self.bump();
            if matches!(self.token, Token::String(_)) {
                let Token::String(name) = self.bump() else {
                    unreachable!()
                };
                let name = ctx.atom_intern(name);
                let body = self.take_body()?;
                return Some(Statement::KeyType(NamedVarDef { merge, name, body }));
            }
            let name = self.parse_lhs_with(ctx, b"type")?;
            return self.parse_variable(ctx, merge, name);
        }
        if self.word(b"key") {
            self.bump();
            if matches!(self.token, Token::Key(_)) {
                let Token::Key(name) = self.bump() else {
                    unreachable!()
                };
                let name = Self::atom(ctx, name);
                let body = self.take_body()?;
                return Some(Statement::Symbols(NamedVarDef { merge, name, body }));
            }
            let name = self.parse_lhs_with(ctx, b"key")?;
            return self.parse_variable(ctx, merge, name);
        }
        if self.word(b"modifier_map") || self.word(b"modmap") || self.word(b"mod_map") {
            self.bump();
            let modifier_word = self.take_word()?;
            let modifier = Self::atom(ctx, modifier_word);
            self.punct(b'{').then_some(())?;
            let mut keys = Vec::new();
            while !self.punct(b'}') {
                keys.push(self.parse_keysym_expr(ctx)?);
                if !self.punct(b',') {
                    self.punct(b'}').then_some(())?;
                    break;
                }
            }
            self.punct(b';');
            return Some(Statement::ModMap(ModMapDef {
                merge,
                modifier,
                keys,
            }));
        }
        if self.word(b"alias") {
            self.bump();
            let Token::Key(alias) = self.bump() else {
                return self.skip_statement();
            };
            self.punct(b'=').then_some(())?;
            let Token::Key(real) = self.bump() else {
                return self.skip_statement();
            };
            let (alias, real) = (Self::atom(ctx, alias), Self::atom(ctx, real));
            self.punct(b';').then_some(())?;
            return Some(Statement::KeyAlias(KeyAliasDef { alias, real }));
        }
        if let Token::Key(_) = self.token {
            let Token::Key(name) = self.bump() else {
                unreachable!()
            };
            let name = Self::atom(ctx, name);
            self.punct(b'=').then_some(())?;
            let Token::Integer(value) = self.bump() else {
                return self.skip_statement();
            };
            self.punct(b';').then_some(())?;
            return Some(Statement::Keycode(KeycodeDef { merge, name, value }));
        }
        if self.word(b"indicator") || self.word(b"virtual") || self.word(b"interpret") {
            self.skip_statement();
            return None;
        }
        let name = if self.punct(b'!') || self.punct(b'~') {
            let word = self.take_word()?;
            let atom = Self::atom(ctx, word);
            self.punct(b';').then_some(())?;
            return Some(Statement::Var(VarDef {
                merge,
                name: Some(Lhs {
                    element: Element::None,
                    field: Self::field(ctx.atom_text(atom).as_bytes()),
                    index: None,
                }),
                value: Some(scalar(Scalar::Integer(0))),
            }));
        } else {
            self.parse_lhs(ctx)?
        };
        self.parse_variable(ctx, merge, name)
    }
    fn parse_variable(
        &mut self,
        ctx: &mut XkbContext,
        merge: MergeMode,
        name: Lhs,
    ) -> Option<Statement<'a>> {
        let value = if self.punct(b';') {
            Some(scalar(Scalar::Integer(1)))
        } else {
            self.punct(b'=').then_some(())?;
            let value = self.parse_expr(ctx, 0);
            self.punct(b';').then_some(())?;
            value
        };
        Some(Statement::Var(VarDef {
            merge,
            name: Some(name),
            value,
        }))
    }
    fn skip_statement(&mut self) -> Option<Statement<'a>> {
        let mut depth = 0;
        loop {
            match self.bump() {
                Token::Punct(b'{') => depth += 1,
                Token::Punct(b'}') if depth > 0 => depth -= 1,
                Token::Punct(b';') if depth == 0 => return Some(Statement::Unknown),
                Token::End | Token::Error => return None,
                _ => {}
            }
        }
    }
    fn take_body(&mut self) -> Option<&'a [u8]> {
        matches!(self.token, Token::Punct(b'{')).then_some(())?;
        let start = self.lexer.pos;
        let end = braced_end(self.lexer.input, start)?;
        self.lexer.pos = end + 1;
        self.token = self.lexer.next();
        self.punct(b';');
        Some(&self.lexer.input[start..end])
    }
    fn parse_body_var(&mut self, ctx: &mut XkbContext) -> Option<VarDef> {
        let merge = self.merge();
        let var = if matches!(self.token, Token::Punct(b'[')) {
            VarDef {
                merge,
                name: None,
                value: Some(self.parse_list(ctx)?),
            }
        } else if self.punct(b'!') || self.punct(b'~') {
            VarDef {
                merge,
                name: Some(Lhs {
                    element: Element::None,
                    field: Self::field(self.take_word()?),
                    index: None,
                }),
                value: Some(scalar(Scalar::Integer(0))),
            }
        } else {
            let name = self.parse_lhs(ctx)?;
            VarDef {
                merge,
                name: Some(name),
                value: if self.punct(b'=') {
                    self.parse_expr(ctx, 0)
                } else {
                    Some(scalar(Scalar::Integer(1)))
                },
            }
        };
        if !self.punct(b',') {
            self.punct(b';');
        }
        Some(var)
    }
    fn parse_lhs(&mut self, ctx: &mut XkbContext) -> Option<Lhs> {
        let first = self.take_word()?;
        self.parse_lhs_with(ctx, first)
    }
    fn parse_lhs_with(&mut self, ctx: &mut XkbContext, first: &[u8]) -> Option<Lhs> {
        let element = Self::element(first);
        let field = Self::field(first);
        let (element, field) = if self.punct(b'.') {
            (element, Self::field(self.take_word()?))
        } else {
            (Element::None, field)
        };
        let index = if self.punct(b'[') {
            let value = self.parse_expr(ctx, 0)?;
            self.punct(b']').then_some(())?;
            Some(value)
        } else {
            None
        };
        Some(Lhs {
            element,
            field,
            index,
        })
    }
    fn element(word: &[u8]) -> Element {
        if word.eq_ignore_ascii_case(b"key") {
            Element::Key
        } else if word.eq_ignore_ascii_case(b"type") {
            Element::Type
        } else {
            Element::Other
        }
    }
    fn field(word: &[u8]) -> Field {
        use Field::*;
        const FIELDS: &[(Field, &[&[u8]])] = &[
            (Type, &[b"type"]),
            (Symbols, &[b"symbols"]),
            (Actions, &[b"actions"]),
            (Vmods, &[b"vmods", b"virtualmods", b"virtualmodifiers"]),
            (Repeat, &[b"repeat", b"repeats", b"repeating"]),
            (Name, &[b"name", b"groupname"]),
            (Modifiers, &[b"modifiers"]),
            (LevelName, &[b"levelname", b"level_name"]),
            (Map, &[b"map"]),
            (Preserve, &[b"preserve"]),
            (
                Ignored,
                &[
                    b"locking",
                    b"lock",
                    b"locks",
                    b"radiogroup",
                    b"permanentradiogroup",
                    b"allownone",
                    b"overlay",
                    b"groupswrap",
                    b"wrapgroups",
                    b"groupsclamp",
                    b"clampgroups",
                    b"groupsredirect",
                    b"redirectgroups",
                ],
            ),
        ];
        for &(field, names) in FIELDS {
            if matches_ci(word, names) {
                return field;
            }
        }
        if word
            .get(..7)
            .is_some_and(|p| p.eq_ignore_ascii_case(b"overlay"))
            || word
                .get(..16)
                .is_some_and(|p| p.eq_ignore_ascii_case(b"permanentoverlay"))
        {
            Ignored
        } else {
            Other
        }
    }
    fn parse_expr(&mut self, ctx: &mut XkbContext, min_precedence: u8) -> Option<ExprKind> {
        let (invert, negative) = if let Token::Punct(op @ (b'-' | b'+' | b'~')) = self.token {
            self.bump();
            (op == b'~', op == b'-')
        } else {
            (false, false)
        };
        let mut left = self.parse_primary(ctx)?;
        if invert || negative {
            let ExprKind::Scalar(expr) = &mut left else {
                return None;
            };
            expr.invert ^= invert;
            if negative {
                for (add, _) in &mut expr.terms {
                    *add = !*add;
                }
            }
        }
        loop {
            let (precedence, add) = match self.token {
                Token::Punct(b'+') => (2, true),
                Token::Punct(b'-') => (2, false),
                _ => break,
            };
            if precedence < min_precedence {
                break;
            }
            self.bump();
            let ExprKind::Scalar(mut right) = self.parse_expr(ctx, precedence + 1)? else {
                return None;
            };
            let ExprKind::Scalar(left) = &mut left else {
                return None;
            };
            for (right_add, term) in right.terms.drain(..) {
                left.terms.try_push((add == right_add, term)).ok()?;
            }
        }
        Some(left)
    }
    fn parse_primary(&mut self, ctx: &mut XkbContext) -> Option<ExprKind> {
        match self.bump() {
            Token::Word(word) => {
                let first = Self::atom(ctx, word);
                if self.punct(b'(') {
                    self.skip_call()?;
                    Some(ExprKind::Actions)
                } else {
                    Some(scalar(Scalar::Ident(first)))
                }
            }
            Token::String(value) => Some(ExprKind::String(ctx.atom_intern(value))),
            Token::Integer(value) => Some(scalar(Scalar::Integer(value))),
            Token::Key(value) => Some(ExprKind::KeyName(Self::atom(ctx, value))),
            Token::Punct(b'[') => self.parse_list_after_open(ctx),
            Token::Punct(b'(') => {
                let value = self.parse_expr(ctx, 0)?;
                self.punct(b')').then_some(value)
            }
            _ => None,
        }
    }
    fn parse_list(&mut self, ctx: &mut XkbContext) -> Option<ExprKind> {
        self.punct(b'[').then_some(())?;
        self.parse_list_after_open(ctx)
    }
    fn parse_list_after_open(&mut self, _ctx: &mut XkbContext) -> Option<ExprKind> {
        if self.punct(b']') {
            return Some(ExprKind::Symbols(Vec::new()));
        }
        let mut items = Vec::new();
        let mut actions = false;
        loop {
            let item = if self.punct(b'{') {
                let mut first_sym = None;
                while !self.punct(b'}') {
                    let next = self.parse_keysym()?;
                    if first_sym.is_none() {
                        first_sym = Some(next);
                    }
                    if !self.punct(b',') {
                        self.punct(b'}').then_some(())?;
                        break;
                    }
                }
                first_sym.unwrap_or(0)
            } else if matches!(self.token, Token::Word(_)) {
                let saved = self.lexer.pos;
                let word = match self.bump() {
                    Token::Word(word) => word,
                    _ => unreachable!(),
                };
                if self.punct(b'(') {
                    actions = true;
                    self.skip_call()?;
                    0
                } else {
                    self.lexer.pos = saved;
                    self.token = Token::Word(word);
                    self.parse_keysym()?
                }
            } else {
                self.parse_keysym()?
            };
            items.push(item);
            if !self.punct(b',') {
                self.punct(b']').then_some(())?;
                break;
            }
        }
        Some(if actions {
            ExprKind::Actions
        } else {
            ExprKind::Symbols(items)
        })
    }
    fn parse_keysym_expr(&mut self, ctx: &mut XkbContext) -> Option<ExprKind> {
        match self.token {
            Token::Key(_) => self.parse_primary(ctx),
            _ => Some(ExprKind::KeySym(self.parse_keysym()?)),
        }
    }
    fn parse_keysym(&mut self) -> Option<u32> {
        Some(match self.bump() {
            Token::Word(word) => resolve_keysym(word).unwrap_or(XKB_KEY_NO_SYMBOL),
            Token::String(value) => std::str::from_utf8(value)
                .ok()?
                .chars()
                .next()
                .and_then(|ch| codepoint_to_keysym(ch as u32))
                .unwrap_or(0),
            Token::Integer(value) if (0..=9).contains(&value) => b'0' as u32 + value as u32,
            Token::Integer(value) if (0..=XKB_KEYSYM_MAX as i64).contains(&value) => value as u32,
            _ => return None,
        })
    }
    fn skip_call(&mut self) -> Option<()> {
        let mut depth = 1;
        while depth != 0 {
            match self.bump() {
                Token::Punct(b'(') => depth += 1,
                Token::Punct(b')') => depth -= 1,
                Token::End | Token::Error => return None,
                _ => {}
            }
        }
        Some(())
    }
}
fn matches_ci(word: &[u8], names: &[&[u8]]) -> bool {
    names.iter().any(|name| word.eq_ignore_ascii_case(name))
}
fn scalar(value: Scalar) -> ExprKind {
    ExprKind::Scalar(ScalarExpr {
        terms: [(true, value)].into_iter().collect(),
        invert: false,
    })
}
fn resolve_keysym(name: &[u8]) -> Option<u32> {
    if !name.is_ascii() {
        let mut chars = std::str::from_utf8(name).ok()?.chars();
        let sym = codepoint_to_keysym(chars.next()? as u32)?;
        return chars.next().is_none().then_some(sym);
    }
    if name.eq_ignore_ascii_case(b"any") || name.eq_ignore_ascii_case(b"nosymbol") {
        return Some(XKB_KEY_NO_SYMBOL);
    }
    if name.eq_ignore_ascii_case(b"none") || name.eq_ignore_ascii_case(b"voidsymbol") {
        return Some(0xffffff);
    }
    if name.len() >= 30 {
        return None;
    }
    let mut buf = [0; 32];
    buf[..name.len()].copy_from_slice(name);
    xkb_keysym_from_name(&buf[..name.len() + 1], XKB_KEYSYM_NO_FLAGS)
}
pub(crate) fn include_create(input: &str, mut merge: MergeMode) -> Option<Vec<IncludeStmt>> {
    let mut items = Vec::new();
    for segment in input.split_inclusive(['+', '|', '^']) {
        let op = segment
            .as_bytes()
            .last()
            .copied()
            .filter(|b| matches!(b, b'+' | b'|' | b'^'));
        let segment = if op.is_some() {
            &segment[..segment.len() - 1]
        } else {
            segment
        };
        let (segment, modifier) = segment.split_once(':').unwrap_or((segment, ""));
        let (file, map) = match segment.split_once('(') {
            Some((file, map)) if !file.is_empty() => (file, map.strip_suffix(')')?),
            Some(_) => return None,
            None => (segment, ""),
        };
        if !file.is_empty() {
            items.push(IncludeStmt {
                merge,
                file: file.into(),
                map: map.into(),
                modifier: modifier.into(),
            });
        }
        merge = match op {
            Some(b'|') => MergeMode::Augment,
            Some(b'^') => MergeMode::Replace,
            _ => MergeMode::Override,
        };
    }
    (!items.is_empty()).then_some(items)
}
fn valid_input(input: &[u8]) -> Option<&[u8]> {
    let input = input.strip_prefix(&[0xef, 0xbb, 0xbf]).unwrap_or(input);
    (input.len() < 2 || input[0].is_ascii() && input[0] != 0 && input[1] != 0).then_some(input)
}
fn select_span(input: &[u8], wanted: &str) -> Option<MapSpan> {
    let input = valid_input(input)?;
    let mut parser = Parser::new(input);
    let mut first = None;
    while !matches!(parser.token, Token::End | Token::Error) {
        let span = parser.scan_file()?;
        if span.file_type == FileType::Ignored {
            continue;
        }
        if !wanted.is_empty() && span.name == wanted
            || wanted.is_empty() && span.flags & MAP_IS_DEFAULT != 0
        {
            return Some(span);
        }
        if first.is_none() {
            first = Some(span);
        }
    }
    first
}
pub(crate) fn xkb_select_map<'a>(input: &'a [u8], wanted: &str) -> Option<SelectedMap<'a>> {
    let input = valid_input(input)?;
    let span = select_span(input, wanted)?;
    Some(SelectedMap {
        file_type: span.file_type,
        body: &input[span.body],
    })
}
pub(crate) fn xkb_select_owned(data: Arc<Vec<u8>>, wanted: &str) -> Option<OwnedMap> {
    let span = select_span(data.as_slice(), wanted)?;
    Some(OwnedMap {
        data,
        body: span.body,
        file_type: span.file_type,
        flags: span.flags,
    })
}
