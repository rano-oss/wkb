#[derive(Default, Debug)]
struct Node {
    next: Vec<(char, usize)>,
    emit: Option<char>,
}

impl Node {
    fn child(&self, c: char) -> Option<usize> {
        self.next.iter().find(|(k, _)| *k == c).map(|(_, i)| *i)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposeState {
    Idle(char),
    Composing(String),
    Finished(char),
    Cancelled(String),
}

pub trait Composer {
    fn feed(&mut self, character: char) -> ComposeState;
}

pub struct ListComposer {
    nodes: Vec<Node>, // nodes[0] = root
    cur: usize,
    pending: Vec<char>,
}

impl ListComposer {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::default()],
            cur: 0,
            pending: Vec::new(),
        }
    }

    pub fn insert(&mut self, pattern: &[char], out: char) {
        let mut n = 0usize;
        for &c in pattern {
            n = match self.nodes[n].child(c) {
                Some(i) => i,
                None => {
                    let i = self.nodes.len();
                    self.nodes.push(Node::default());
                    self.nodes[n].next.push((c, i));
                    i
                }
            };
        }
        self.nodes[n].emit = Some(out);
    }
}

impl Composer for ListComposer {
    fn feed(&mut self, character: char) -> ComposeState {
        if self.pending.is_empty() {
            if let Some(next) = self.nodes[0].child(character) {
                if let Some(outc) = self.nodes[next].emit {
                    ComposeState::Finished(outc)
                } else {
                    self.cur = next;
                    self.pending.push(character);
                    ComposeState::Composing(character.to_string())
                }
            } else {
                ComposeState::Idle(character)
            }
        } else {
            if let Some(next) = self.nodes[self.cur].child(character) {
                self.pending.push(character);
                if let Some(outc) = self.nodes[next].emit {
                    self.cur = 0;
                    self.pending.clear();
                    ComposeState::Finished(outc)
                } else {
                    self.cur = next;
                    ComposeState::Composing(self.pending.iter().collect())
                }
            } else {
                self.pending.push(character);
                let failed: String = self.pending.iter().collect();
                self.cur = 0;
                self.pending.clear();
                ComposeState::Cancelled(failed)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnicodeComposer {
    pub(crate) pending: Vec<char>,
}

impl UnicodeComposer {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
        }
    }
}

impl Composer for UnicodeComposer {
    fn feed(&mut self, character: char) -> ComposeState {
        if unicode_normalization::char::is_combining_mark(character) {
            self.pending.push(character);
            ComposeState::Composing(self.pending.iter().collect())
        } else {
            if self.pending.is_empty() {
                return ComposeState::Idle(character);
            }

            let mut current = character;
            let mut success = true;
            for &mark in &self.pending {
                if let Some(c) = unicode_normalization::char::compose(current, mark) {
                    current = c;
                } else {
                    success = false;
                    break;
                }
            }

            if success {
                self.pending.clear();
                ComposeState::Finished(current)
            } else {
                let mut s: String = self.pending.iter().collect();
                s.push(character);
                self.pending.clear();
                ComposeState::Cancelled(s)
            }
        }
    }
}
