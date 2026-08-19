use super::keysym::xkb_keysym_from_name;
use super::parser::*;
use crate::xkb::keysym::codepoint_to_keysym;
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
struct Parser<'a, 'ctx> {
    ctx: &'ctx mut XkbContext,
    lexer: Lexer<'a>,
    token: Token<'a>,
}
impl<'a, 'ctx> Parser<'a, 'ctx> {
    fn new(ctx: &'ctx mut XkbContext, input: &'a [u8]) -> Self {
        let mut lexer = Lexer::new(input);
        let token = lexer.next();
        Self { ctx, lexer, token }
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
    fn atom(&mut self, word: &[u8]) -> u32 {
        self.ctx.atom_intern(word)
    }
    fn parse_input(&mut self, wanted: &str) -> Option<XkbFile> {
        let mut first = None;
        while !matches!(self.token, Token::End | Token::Error) {
            let file =
                self.parse_file((!wanted.is_empty() && first.is_some()).then_some(wanted))?;
            if file.file_type == FileType::Ignored {
                continue;
            }
            if (!wanted.is_empty() && file.name == wanted)
                || (wanted.is_empty() && file.flags & MAP_IS_DEFAULT != 0)
            {
                return Some(file);
            }
            if first.is_none() {
                first = Some(file);
            }
        }
        first
    }
    fn parse_file(&mut self, wanted: Option<&str>) -> Option<XkbFile> {
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
        let kind = match self.take_word()? {
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
                || word.eq_ignore_ascii_case(b"xkb_compatibility_map") =>
            {
                FileType::Ignored
            }
            word if word.eq_ignore_ascii_case(b"xkb_geometry") => FileType::Ignored,
            _ => return None,
        };
        let name = match self.token {
            Token::String(_) => match self.bump() {
                Token::String(name) => name,
                _ => unreachable!(),
            },
            _ => String::new(),
        };
        self.punct(b'{').then_some(())?;
        if wanted.is_some_and(|wanted| name != wanted) {
            self.skip_block()?;
            self.punct(b';');
            return Some(xkb_file_create(FileType::Ignored, name, Vec::new(), flags));
        }
        let mut defs = Vec::new();
        if kind == FileType::Keymap {
            while !self.punct(b'}') {
                defs.push(Statement::XkbFile(self.parse_file(None)?));
                self.punct(b';');
            }
        } else if kind == FileType::Ignored {
            self.skip_block()?;
        } else {
            while !self.punct(b'}') {
                if matches!(self.token, Token::End | Token::Error) {
                    return None;
                }
                if let Some(statement) = self.parse_statement() {
                    defs.push(statement);
                }
            }
        }
        self.punct(b';');
        Some(xkb_file_create(kind, name, defs, flags))
    }
    fn skip_block(&mut self) -> Option<()> {
        let mut depth = 1;
        while depth != 0 {
            match self.bump() {
                Token::Punct(b'{') => depth += 1,
                Token::Punct(b'}') => depth -= 1,
                Token::End | Token::Error => return None,
                _ => {}
            }
        }
        Some(())
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
    fn parse_statement(&mut self) -> Option<Statement> {
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
                let name = self.atom(name_word);
                let value = self.punct(b'=').then(|| self.parse_expr(0)).flatten();
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
                let name = self.ctx.atom_intern(name.as_bytes());
                let body = self.parse_body()?;
                return Some(Statement::KeyType(NamedVarDef { merge, name, body }));
            }
            let atom = self.ctx.atom_intern(b"type");
            let name = self.parse_word_tail(atom)?;
            return self.parse_variable(merge, name);
        }
        if self.word(b"key") {
            self.bump();
            if matches!(self.token, Token::Key(_)) {
                let Token::Key(name) = self.bump() else {
                    unreachable!()
                };
                let name = self.atom(name);
                let body = self.parse_body()?;
                return Some(Statement::Symbols(NamedVarDef { merge, name, body }));
            }
            let atom = self.ctx.atom_intern(b"key");
            let name = self.parse_word_tail(atom)?;
            return self.parse_variable(merge, name);
        }
        if self.word(b"modifier_map") || self.word(b"modmap") || self.word(b"mod_map") {
            self.bump();
            let modifier_word = self.take_word()?;
            let modifier = self.atom(modifier_word);
            self.punct(b'{').then_some(())?;
            let mut keys = Vec::new();
            while !self.punct(b'}') {
                keys.push(self.parse_keysym_expr()?);
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
            let (alias, real) = (self.atom(alias), self.atom(real));
            self.punct(b';').then_some(())?;
            return Some(Statement::KeyAlias(KeyAliasDef { merge, alias, real }));
        }
        if let Token::Key(_) = self.token {
            let Token::Key(name) = self.bump() else {
                unreachable!()
            };
            let name = self.atom(name);
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
            let atom = self.atom(word);
            self.punct(b';').then_some(())?;
            return Some(Statement::Var(VarDef {
                merge,
                name: Some(ExprKind::Ident(atom)),
                value: Some(ExprKind::Integer(0)),
            }));
        } else {
            self.parse_expr(2)?
        };
        self.parse_variable(merge, name)
    }
    fn parse_variable(&mut self, merge: MergeMode, name: ExprKind) -> Option<Statement> {
        let value = if self.punct(b';') {
            Some(ExprKind::Integer(1))
        } else {
            self.punct(b'=').then_some(())?;
            let value = self.parse_expr(0);
            self.punct(b';').then_some(())?;
            value
        };
        Some(Statement::Var(VarDef {
            merge,
            name: Some(name),
            value,
        }))
    }
    fn skip_statement(&mut self) -> Option<Statement> {
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
    fn parse_body(&mut self) -> Option<Vec<VarDef>> {
        self.punct(b'{').then_some(())?;
        let mut body = Vec::new();
        while !self.punct(b'}') {
            let merge = self.merge();
            if matches!(self.token, Token::Punct(b'[')) {
                let value = self.parse_list()?;
                body.push(VarDef {
                    merge,
                    name: None,
                    value: Some(value),
                });
            } else if self.punct(b'!') || self.punct(b'~') {
                let word = self.take_word()?;
                let atom = self.atom(word);
                body.push(VarDef {
                    merge,
                    name: Some(ExprKind::Ident(atom)),
                    value: Some(ExprKind::Integer(0)),
                });
            } else {
                let name = self.parse_expr(2)?;
                if self.punct(b'=') {
                    body.push(VarDef {
                        merge,
                        name: Some(name),
                        value: self.parse_expr(0),
                    });
                } else {
                    body.push(VarDef {
                        merge,
                        name: Some(name),
                        value: Some(ExprKind::Integer(1)),
                    });
                }
            }
            if !self.punct(b',') {
                self.punct(b';');
            }
        }
        self.punct(b';');
        Some(body)
    }
    fn parse_expr(&mut self, min_precedence: u8) -> Option<ExprKind> {
        let mut left = if let Token::Punct(op @ (b'-' | b'+' | b'!' | b'~')) = self.token {
            self.bump();
            ExprKind::Unary {
                op: match op {
                    b'-' => UnaryOp::Negate,
                    b'+' => UnaryOp::Plus,
                    b'!' => UnaryOp::Not,
                    _ => UnaryOp::Invert,
                },
                child: Box::new(self.parse_expr(4)?),
            }
        } else {
            self.parse_primary()?
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
            let right = self.parse_expr(precedence + u8::from(op != BinaryOp::Assign))?;
            left = ExprKind::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Some(left)
    }
    fn parse_primary(&mut self) -> Option<ExprKind> {
        match self.bump() {
            Token::Word(word) => {
                let first = self.atom(word);
                self.parse_word_tail(first)
            }
            Token::String(value) => Some(ExprKind::String(self.ctx.atom_intern(value.as_bytes()))),
            Token::Integer(value) => Some(ExprKind::Integer(value)),
            Token::Float => None,
            Token::Key(value) => Some(ExprKind::KeyName(self.atom(value))),
            Token::Punct(b'[') => self.parse_list_after_open(),
            Token::Punct(b'(') => {
                let value = self.parse_expr(0)?;
                self.punct(b')').then_some(value)
            }
            _ => None,
        }
    }
    fn parse_word_tail(&mut self, first: u32) -> Option<ExprKind> {
        if self.punct(b'(') {
            let mut args = Vec::new();
            while !self.punct(b')') {
                args.push(self.parse_expr(0)?);
                if !self.punct(b',') {
                    self.punct(b')').then_some(())?;
                    break;
                }
            }
            Some(ExprKind::Action { name: first, args })
        } else if self.punct(b'.') {
            let field_word = self.take_word()?;
            let field = self.atom(field_word);
            let index = if self.punct(b'[') {
                let index = self.parse_expr(0)?;
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
            let index = self.parse_expr(0)?;
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
    fn parse_list(&mut self) -> Option<ExprKind> {
        self.punct(b'[').then_some(())?;
        self.parse_list_after_open()
    }
    fn parse_list_after_open(&mut self) -> Option<ExprKind> {
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
                    let name = self.atom(word);
                    let mut args = Vec::new();
                    while !self.punct(b')') {
                        args.push(self.parse_expr(0)?);
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
    fn parse_keysym_expr(&mut self) -> Option<ExprKind> {
        match self.token {
            Token::Key(_) => self.parse_primary(),
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
fn include_create(input: &str, mut merge: MergeMode) -> Option<Vec<IncludeStmt>> {
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
fn xkb_file_create(kind: FileType, name: String, defs: Vec<Statement>, flags: u32) -> XkbFile {
    XkbFile {
        file_type: kind,
        name,
        defs,
        flags,
    }
}
pub(crate) fn xkb_parse_string(ctx: &mut XkbContext, input: &[u8], map: &str) -> Option<XkbFile> {
    let input = input.strip_prefix(&[0xef, 0xbb, 0xbf]).unwrap_or(input);
    if input.len() >= 2 && (!input[0].is_ascii() || input[0] == 0 || input[1] == 0) {
        return None;
    }
    Parser::new(ctx, input).parse_input(map)
}
pub(crate) fn xkb_file_from_components(parts: &XkbComponentNames) -> Option<XkbFile> {
    let mut defs = Vec::new();
    for (kind, bytes) in [
        (FileType::Keycodes, &parts.keycodes),
        (FileType::Types, &parts.types),
        (FileType::Symbols, &parts.symbols),
    ] {
        let input = std::str::from_utf8(bytes).ok()?;
        let include = include_create(input, MergeMode::Default)?;
        defs.push(Statement::XkbFile(xkb_file_create(
            kind,
            String::new(),
            vec![Statement::Include(include)],
            0,
        )));
    }
    Some(xkb_file_create(FileType::Keymap, String::new(), defs, 0))
}
