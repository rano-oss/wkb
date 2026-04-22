/// Token fed into the composer: either a regular character or a Compose key press
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Char(char),
    Compose,
}

pub trait Composer: std::fmt::Debug {
    fn feed(&mut self, token: Token) -> ComposeState;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposeState {
    Idle(Token),
    Composing(Vec<Token>),
    Finished(char),
    Cancelled(Vec<Token>),
}

#[derive(Debug, Clone)]
pub(crate) enum Node {
    Next(Vec<(Token, usize)>),
    Emit(char),
}

#[derive(Debug, Clone)]
pub struct ListComposer {
    pub(crate) nodes: Vec<Node>,
    cur: usize,
    pending: Vec<Token>,
}

impl Default for ListComposer {
    fn default() -> Self {
        Self::new()
    }
}

impl ListComposer {
    pub fn new() -> Self {
        Self {
            // root exists and is a Next node
            nodes: vec![Node::Next(Vec::new())],
            cur: 0,
            pending: Vec::new(),
        }
    }

    /// Insert a sequence of tokens into the trie. Tokens are either `Token::Char` or `Token::Compose`.
    pub fn insert(&mut self, tokens: &[Token], out: char) {
        let mut n = 0usize;
        for t in tokens.iter() {
            let i = self.nodes.len();
            n = match self.nodes[n] {
                Node::Next(ref mut items) => {
                    if let Some((_, next)) = items.iter().find(|(k, _)| k == t) {
                        *next
                    } else {
                        items.push((t.clone(), i));
                        self.nodes.push(Node::Next(Vec::new()));
                        i
                    }
                }
                Node::Emit(_) => {
                    self.nodes[n] = Node::Next(vec![(t.clone(), i)]);
                    self.nodes.push(Node::Next(Vec::new()));
                    i
                }
            };
        }
        self.nodes[n] = Node::Emit(out);
    }
}

impl Composer for ListComposer {
    fn feed(&mut self, token: Token) -> ComposeState {
        match &self.nodes[self.cur] {
            Node::Next(items) => {
                if let Some((_, next)) = items.iter().find(|(k, _)| k == &token) {
                    self.pending.push(token);
                    self.cur = *next;
                    // Check if we landed on an Emit node
                    match &self.nodes[self.cur] {
                        Node::Emit(outc) => {
                            let out = *outc;
                            self.cur = 0;
                            self.pending.clear();
                            ComposeState::Finished(out)
                        }
                        _ => ComposeState::Composing(self.pending.clone()),
                    }
                } else if self.cur == 0 {
                    ComposeState::Idle(token)
                } else {
                    self.pending.push(token);
                    self.cur = 0;
                    let failed = self.pending.drain(..);
                    ComposeState::Cancelled(failed.collect())
                }
            }
            Node::Emit(outc) => {
                self.cur = 0;
                self.pending.clear();
                ComposeState::Finished(*outc)
            }
        }
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct UnicodeComposer {
//     pub(crate) pending: Vec<char>,
// }

// impl UnicodeComposer {
//     pub fn new() -> Self {
//         Self {
//             pending: Vec::new(),
//         }
//     }
// }

// impl Composer for UnicodeComposer {
//     fn feed(&mut self, character: char) -> ComposeState {
//         if unicode_normalization::char::is_combining_mark(character) {
//             self.pending.push(character);
//             ComposeState::Composing(self.pending.iter().collect())
//         } else {
//             if self.pending.is_empty() {
//                 return ComposeState::Idle(character);
//             }

//             let mut current = character;
//             let mut success = true;
//             for &mark in &self.pending {
//                 if let Some(c) = unicode_normalization::char::compose(current, mark) {
//                     current = c;
//                 } else {
//                     success = false;
//                     break;
//                 }
//             }

//             if success {
//                 self.pending.clear();
//                 ComposeState::Finished(current)
//             } else {
//                 let mut s: String = self.pending.iter().collect();
//                 s.push(character);
//                 self.pending.clear();
//                 ComposeState::Cancelled(s)
//             }
//         }
//     }
// }
