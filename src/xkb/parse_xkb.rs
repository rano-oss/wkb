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
    String(String),
    Key(&'a [u8]),
    Integer(i64),
    Float,
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
            let mut value = String::new();
            while let Some(&byte) = self.input.get(self.pos) {
                self.pos += 1;
                match byte {
                    b'"' => return Token::String(value),
                    b'\n' => return Token::Error,
                    b'\\' => {
                        let escaped = match self.input.get(self.pos).copied() {
                            Some(byte) => byte,
                            None => return Token::Error,
                        };
                        self.pos += 1;
                        match escaped {
                            b'n' => value.push('\n'),
                            b't' => value.push('\t'),
                            b'r' => value.push('\r'),
                            b'b' => value.push('\x08'),
                            b'f' => value.push('\x0c'),
                            b'v' => value.push('\x0b'),
                            b'e' => value.push('\x1b'),
                            b'u' if self.input.get(self.pos) == Some(&b'{') => {
                                self.pos += 1;
                                let start = self.pos;
                                while self.input.get(self.pos).is_some_and(u8::is_ascii_hexdigit) {
                                    self.pos += 1;
                                }
                                let code = std::str::from_utf8(&self.input[start..self.pos])
                                    .ok()
                                    .and_then(|s| u32::from_str_radix(s, 16).ok())
                                    .and_then(char::from_u32);
                                if self.input.get(self.pos) != Some(&b'}') || code.is_none() {
                                    return Token::Error;
                                }
                                self.pos += 1;
                                value.push(code.unwrap());
                            }
                            b'0'..=b'7' => {
                                let mut number = (escaped - b'0') as u32;
                                for _ in 1..4 {
                                    let Some(next @ b'0'..=b'7') =
                                        self.input.get(self.pos).copied()
                                    else {
                                        break;
                                    };
                                    self.pos += 1;
                                    number = number * 8 + (next - b'0') as u32;
                                }
                                if let Some(ch) = char::from_u32(number) {
                                    value.push(ch);
                                }
                            }
                            byte => value.push(byte as char),
                        }
                    }
                    byte if byte.is_ascii() => value.push(byte as char),
                    _ => {
                        let start = self.pos - 1;
                        let Ok(text) = std::str::from_utf8(&self.input[start..]) else {
                            return Token::Error;
                        };
                        let Some(ch) = text.chars().next() else {
                            return Token::Error;
                        };
                        self.pos = start + ch.len_utf8();
                        value.push(ch);
                    }
                }
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
            if !hex && self.input.get(self.pos) == Some(&b'.') {
                self.pos += 1;
                self.take_while(|byte| byte.is_ascii_digit());
                return Token::Float;
            }
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
    data: Arc<[u8]>,
    body: std::ops::Range<usize>,
    pub(crate) file_type: FileType,
    pub(crate) flags: u32,
}
pub(crate) struct Stream<'a> {
    parser: Parser<'a>,
}
impl OwnedMap {
    pub(crate) fn stream(&self) -> Stream<'_> {
        Stream::new(&self.data[self.body.clone()])
    }
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
                flags |= MAP_HAS_MAP_FLAGS;
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
                Token::String(name) => name,
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
            return include_create(&value, merge).map(Statement::Include);
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
                let name = ctx.atom_intern(name.as_bytes());
                let body = self.take_body()?;
                return Some(Statement::KeyType(NamedVarDef { merge, name, body }));
            }
            let atom = ctx.atom_intern(b"type");
            let name = self.parse_word_tail(ctx, atom)?;
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
            let atom = ctx.atom_intern(b"key");
            let name = self.parse_word_tail(ctx, atom)?;
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
            return Some(Statement::KeyAlias(KeyAliasDef { merge, alias, real }));
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
                name: Some(ExprKind::Ident(atom)),
                value: Some(ExprKind::Integer(0)),
            }));
        } else {
            self.parse_expr(ctx, 2)?
        };
        self.parse_variable(ctx, merge, name)
    }
    fn parse_variable(
        &mut self,
        ctx: &mut XkbContext,
        merge: MergeMode,
        name: ExprKind,
    ) -> Option<Statement<'a>> {
        let value = if self.punct(b';') {
            Some(ExprKind::Integer(1))
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
                name: Some(ExprKind::Ident(Self::atom(ctx, self.take_word()?))),
                value: Some(ExprKind::Integer(0)),
            }
        } else {
            let name = self.parse_expr(ctx, 2)?;
            VarDef {
                merge,
                name: Some(name),
                value: if self.punct(b'=') {
                    self.parse_expr(ctx, 0)
                } else {
                    Some(ExprKind::Integer(1))
                },
            }
        };
        if !self.punct(b',') {
            self.punct(b';');
        }
        Some(var)
    }
    fn parse_expr(&mut self, ctx: &mut XkbContext, min_precedence: u8) -> Option<ExprKind> {
        let mut left = if let Token::Punct(op @ (b'-' | b'+' | b'!' | b'~')) = self.token {
            self.bump();
            ExprKind::Unary {
                op: match op {
                    b'-' => UnaryOp::Negate,
                    b'+' => UnaryOp::Plus,
                    b'!' => UnaryOp::Not,
                    _ => UnaryOp::Invert,
                },
                child: Box::new(self.parse_expr(ctx, 4)?),
            }
        } else {
            self.parse_primary(ctx)?
        };
        loop {
            let (precedence, op) = match self.token {
                Token::Punct(b'=') => (1, BinaryOp::Assign),
                Token::Punct(b'+') => (2, BinaryOp::Add),
                Token::Punct(b'-') => (2, BinaryOp::Subtract),
                Token::Punct(b'*') => (3, BinaryOp::Multiply),
                Token::Punct(b'/') => (3, BinaryOp::Divide),
                _ => break,
            };
            if precedence < min_precedence {
                break;
            }
            self.bump();
            let right = self.parse_expr(ctx, precedence + u8::from(op != BinaryOp::Assign))?;
            left = ExprKind::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Some(left)
    }
    fn parse_primary(&mut self, ctx: &mut XkbContext) -> Option<ExprKind> {
        match self.bump() {
            Token::Word(word) => {
                let first = Self::atom(ctx, word);
                self.parse_word_tail(ctx, first)
            }
            Token::String(value) => Some(ExprKind::String(ctx.atom_intern(value.as_bytes()))),
            Token::Integer(value) => Some(ExprKind::Integer(value)),
            Token::Float => None,
            Token::Key(value) => Some(ExprKind::KeyName(Self::atom(ctx, value))),
            Token::Punct(b'[') => self.parse_list_after_open(ctx),
            Token::Punct(b'(') => {
                let value = self.parse_expr(ctx, 0)?;
                self.punct(b')').then_some(value)
            }
            _ => None,
        }
    }
    fn parse_word_tail(&mut self, ctx: &mut XkbContext, first: u32) -> Option<ExprKind> {
        if self.punct(b'(') {
            let mut args = Vec::new();
            while !self.punct(b')') {
                args.push(self.parse_expr(ctx, 0)?);
                if !self.punct(b',') {
                    self.punct(b')').then_some(())?;
                    break;
                }
            }
            Some(ExprKind::Action { name: first, args })
        } else if self.punct(b'.') {
            let field_word = self.take_word()?;
            let field = Self::atom(ctx, field_word);
            let index = if self.punct(b'[') {
                let index = self.parse_expr(ctx, 0)?;
                self.punct(b']').then_some(())?;
                Some(Box::new(index))
            } else {
                None
            };
            Some(ExprKind::FieldRef {
                element: first,
                field,
                index,
            })
        } else if self.punct(b'[') {
            let index = self.parse_expr(ctx, 0)?;
            self.punct(b']').then_some(())?;
            Some(ExprKind::FieldRef {
                element: 0,
                field: first,
                index: Some(Box::new(index)),
            })
        } else {
            Some(ExprKind::Ident(first))
        }
    }
    fn parse_list(&mut self, ctx: &mut XkbContext) -> Option<ExprKind> {
        self.punct(b'[').then_some(())?;
        self.parse_list_after_open(ctx)
    }
    fn parse_list_after_open(&mut self, ctx: &mut XkbContext) -> Option<ExprKind> {
        if self.punct(b']') {
            return Some(ExprKind::EmptyList);
        }
        let mut items = Vec::new();
        loop {
            let item = if self.punct(b'{') {
                let mut syms = Vec::new();
                while !self.punct(b'}') {
                    self.append_keysym(&mut syms)?;
                    if !self.punct(b',') {
                        self.punct(b'}').then_some(())?;
                        break;
                    }
                }
                ExprKind::KeysymList { syms }
            } else if matches!(self.token, Token::Word(_)) {
                let saved = self.lexer.pos;
                let word = match self.bump() {
                    Token::Word(word) => word,
                    _ => unreachable!(),
                };
                if self.punct(b'(') {
                    let name = Self::atom(ctx, word);
                    let mut args = Vec::new();
                    while !self.punct(b')') {
                        args.push(self.parse_expr(ctx, 0)?);
                        if !self.punct(b',') {
                            self.punct(b')').then_some(())?;
                            break;
                        }
                    }
                    ExprKind::ActionList {
                        actions: vec![ExprKind::Action { name, args }],
                    }
                } else {
                    self.lexer.pos = saved;
                    self.token = Token::Word(word);
                    let mut syms = Vec::new();
                    self.append_keysym(&mut syms)?;
                    ExprKind::KeysymList { syms }
                }
            } else {
                let mut syms = Vec::new();
                self.append_keysym(&mut syms)?;
                ExprKind::KeysymList { syms }
            };
            items.push(item);
            if !self.punct(b',') {
                self.punct(b']').then_some(())?;
                break;
            }
        }
        Some(ExprKind::ActionList { actions: items })
    }
    fn parse_keysym_expr(&mut self, ctx: &mut XkbContext) -> Option<ExprKind> {
        match self.token {
            Token::Key(_) => self.parse_primary(ctx),
            _ => {
                let mut syms = Vec::new();
                self.append_keysym(&mut syms)?;
                syms.first()
                    .copied()
                    .map(ExprKind::KeySym)
                    .or(Some(ExprKind::KeySym(0)))
            }
        }
    }
    fn append_keysym(&mut self, syms: &mut Vec<u32>) -> Option<()> {
        match self.bump() {
            Token::Word(word) => {
                let sym = resolve_keysym(word).unwrap_or(XKB_KEY_NO_SYMBOL);
                if sym != XKB_KEY_NO_SYMBOL {
                    syms.push(sym);
                }
            }
            Token::String(value) => {
                for ch in value.chars() {
                    let sym = codepoint_to_keysym(ch as u32)?;
                    if sym != XKB_KEY_NO_SYMBOL {
                        syms.push(sym);
                    }
                }
            }
            Token::Integer(value) if (0..=9).contains(&value) => {
                syms.push(b'0' as u32 + value as u32)
            }
            Token::Integer(value) if (0..=XKB_KEYSYM_MAX as i64).contains(&value) => {
                syms.push(value as u32)
            }
            _ => return None,
        }
        Some(())
    }
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
pub(crate) fn xkb_select_owned(data: Arc<[u8]>, wanted: &str) -> Option<OwnedMap> {
    let span = select_span(&data, wanted)?;
    Some(OwnedMap {
        data,
        body: span.body,
        file_type: span.file_type,
        flags: span.flags,
    })
}
