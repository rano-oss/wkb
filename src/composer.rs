use arrayvec::ArrayString;

/// Token fed into the composer: either a regular character or a Compose key press
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Token {
    Char(char),
    Compose,
}

/// Compose sequence display string — fixed-size, stack-only, no allocation.
pub type ComposeString = ArrayString<16>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposeState {
    Idle(char),
    Composing(ComposeString),
    Finished(char),
    Cancelled,
}

/// Convert a Token to a u32 key for fast comparison.
/// Compose = 0, Char(c) = c as u32 (always >= 1 for valid chars).
#[inline(always)]
fn token_key(token: &Token) -> u32 {
    match token {
        Token::Compose => 0,
        Token::Char(c) => *c as u32,
    }
}

/// Trie node: children stored as sorted (key, child_index) pairs for binary search.
/// `emit` is Some(char) if this node is a leaf that produces output.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct TrieNode {
    pub(crate) children: Vec<(u32, u32)>, // (token_key, node_index), sorted by token_key
    pub(crate) emit: Option<char>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Composer {
    pub(crate) nodes: Vec<TrieNode>,
    #[serde(skip)]
    cur: u32,
    #[serde(skip)]
    buf: ComposeString,
}

impl Default for Composer {
    fn default() -> Self {
        Self::new()
    }
}

impl Composer {
    pub fn new() -> Self {
        Self {
            nodes: vec![TrieNode {
                children: Vec::new(),
                emit: None,
            }],
            cur: 0,
            buf: ComposeString::new(),
        }
    }

    /// Insert a sequence of tokens into the trie.
    pub fn insert(&mut self, tokens: &[Token], out: char) {
        let mut n = 0u32;
        for t in tokens.iter() {
            let key = token_key(t);
            let children = &self.nodes[n as usize].children;
            match children.binary_search_by_key(&key, |&(k, _)| k) {
                Ok(pos) => {
                    n = children[pos].1;
                }
                Err(pos) => {
                    let new_idx = self.nodes.len() as u32;
                    self.nodes.push(TrieNode {
                        children: Vec::new(),
                        emit: None,
                    });
                    self.nodes[n as usize].children.insert(pos, (key, new_idx));
                    n = new_idx;
                }
            }
        }
        self.nodes[n as usize].emit = Some(out);
    }

    #[inline]
    pub(crate) fn feed(&mut self, token: Token) -> ComposeState {
        let key = token_key(&token);
        let node = &self.nodes[self.cur as usize];

        match node.children.binary_search_by_key(&key, |&(k, _)| k) {
            Ok(pos) => {
                let next = node.children[pos].1;
                let next_node = &self.nodes[next as usize];
                if let Some(out) = next_node.emit {
                    self.cur = 0;
                    self.buf.clear();
                    ComposeState::Finished(out)
                } else {
                    self.cur = next;
                    match token {
                        Token::Char(c) => {
                            let _ = self.buf.try_push(c);
                            ComposeState::Composing(self.buf)
                        }
                        Token::Compose => {
                            let mut display = self.buf;
                            let _ = display.try_push('·');
                            ComposeState::Composing(display)
                        }
                    }
                }
            }
            Err(_) => {
                if self.cur == 0 {
                    match token {
                        Token::Compose => ComposeState::Idle('·'),
                        Token::Char(c) => ComposeState::Idle(c),
                    }
                } else {
                    self.cur = 0;
                    self.buf.clear();
                    ComposeState::Cancelled
                }
            }
        }
    }

    pub(crate) fn reset(&mut self) {
        self.cur = 0;
        self.buf.clear();
    }
}
