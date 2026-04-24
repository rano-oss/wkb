/// Token fed into the composer: either a regular character or a Compose key press
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Char(char),
    Compose,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposeState {
    Idle(char),
    Composing(String),
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
#[derive(Debug, Clone)]
pub(crate) struct TrieNode {
    children: Vec<(u32, u32)>, // (token_key, node_index), sorted by token_key
    emit: Option<char>,
}

#[derive(Debug, Clone)]
pub struct Composer {
    pub(crate) nodes: Vec<TrieNode>,
    cur: u32,
    pending: Vec<Token>,
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
            pending: Vec::new(),
        }
    }

    /// Returns the tokens fed so far in the current compose sequence.
    /// Non-empty while in `Composing` state. Clients can use this to
    /// display the in-progress sequence (e.g. `'` `¨` with underline).
    #[inline]
    pub fn pending(&self) -> &[Token] {
        &self.pending
    }

    /// Insert a sequence of tokens into the trie.
    pub fn insert(&mut self, tokens: &[Token], out: char) {
        let mut n = 0u32;
        for t in tokens.iter() {
            let key = token_key(t);
            let children = &self.nodes[n as usize].children;
            // Binary search for existing child
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

    /// Returns an opinionated display string of the in-progress compose sequence.
    /// Compose key shows as `·` if it is the last token pressed.
    /// Characters show as themselves.
    pub fn pending_string(&self) -> String {
        if self.pending.is_empty() {
            return String::new();
        }

        let mut s = String::with_capacity(self.pending.len());
        let last = self.pending.len() - 1;

        for token in &self.pending[..last] {
            if let Token::Char(c) = token {
                s.push(*c);
            }
        }

        match self.pending[last] {
            Token::Compose => s.push('·'),
            Token::Char(c) => s.push(c),
        }

        s
    }

    #[inline]
    pub(crate) fn feed(&mut self, token: Token) -> ComposeState {
        let key = token_key(&token);
        let node = &self.nodes[self.cur as usize];

        match node.children.binary_search_by_key(&key, |&(k, _)| k) {
            Ok(pos) => {
                let next = node.children[pos].1;
                self.pending.push(token);
                let next_node = &self.nodes[next as usize];
                if let Some(out) = next_node.emit {
                    self.cur = 0;
                    self.pending.clear();
                    ComposeState::Finished(out)
                } else {
                    self.cur = next;
                    ComposeState::Composing(self.pending_string())
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
                    self.pending.clear();
                    ComposeState::Cancelled
                }
            }
        }
    }

    pub(crate) fn reset(&mut self) {
        self.cur = 0;
        self.pending.clear();
    }
}
