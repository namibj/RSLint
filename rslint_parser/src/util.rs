//! Extra utlities for untyped syntax nodes, syntax tokens, and AST nodes.

use crate::*;

pub use rslint_lexer::color;

/// Extensions to rowan's SyntaxNode
pub trait SyntaxNodeExt {
    #[doc(hidden)]
    fn to_node(&self) -> &SyntaxNode;

    /// Get all of the tokens of this node, recursively, including whitespace and comments.
    fn tokens(&self) -> Vec<SyntaxToken> {
        self.to_node()
            .descendants_with_tokens()
            .filter_map(|x| x.into_token())
            .collect()
    }

    /// Get all the tokens of this node, recursively, not including whitespace and comments.
    fn lossy_tokens(&self) -> Vec<SyntaxToken> {
        self.to_node()
            .descendants_with_tokens()
            .filter_map(|x| x.into_token().filter(|token| !token.kind().is_trivia()))
            .collect()
    }

    /// Get the first non-whitespace child token.
    fn first_lossy_token(&self) -> Option<SyntaxToken> {
        self.to_node()
            .children_with_tokens()
            .filter_map(|it| it.into_token().filter(|x| !x.kind().is_trivia()))
            .next()
    }

    /// Check if the node is a certain AST node and that it can be casted to it.
    fn is<T: AstNode>(&self) -> bool {
        T::can_cast(self.to_node().kind())
    }

    /// Cast this node to a certain AST node.
    ///
    /// # Panics
    /// Panics if the underlying node cannot be cast to the AST node
    fn to<T: AstNode>(&self) -> T {
        T::cast(self.to_node().to_owned()).unwrap_or_else(|| {
            panic!(
                "Tried to cast node as `{:?}` but was unable to cast",
                stringify!(T)
            )
        })
    }

    /// Try to cast this node to a certain AST node
    fn try_to<T: AstNode>(&self) -> Option<T> {
        T::cast(self.to_node().to_owned())
    }

    /// Compare two syntax nodes by comparing their underlying non-whitespace tokens.
    ///
    /// This is a more accurate way of comparing nodes because it does not count whitespace.
    /// Text based equality counts `foo. bar` and `foo.bar` as different, while this counts them as the same.
    ///
    /// # Examples
    ///
    /// ```
    /// use rslint_parser::{SyntaxNodeExt, parse_expr};
    ///
    /// let left = parse_expr("foo. bar", 0).syntax();
    /// let right = parse_expr("foo.bar", 0).syntax();
    ///
    /// assert!(left.lexical_eq(&right));
    ///
    /// assert_ne!(left.text(), right.text());
    /// ```
    fn lexical_eq(&self, right: &SyntaxNode) -> bool {
        let left = self.lossy_tokens();
        let right = right.lossy_tokens();

        if left.len() == right.len() {
            left.iter()
                .zip(right.iter())
                .all(|(l, r)| l.text() == r.text())
        } else {
            false
        }
    }

    /// Syntax highlight the node's text into an ANSI string.
    /// If stdout and stderr are not terminals, this will return the raw
    /// node text.
    fn color(&self) -> String {
        color(&self.to_node().text().to_string())
    }

    /// Get the text range of this node, not including any leading or trailing whitespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use rslint_parser::{SyntaxNodeExt, parse_expr, TextRange};
    ///
    /// let node = parse_expr(" foo. bar  ", 0).syntax();
    ///
    /// assert_eq!(node.trimmed_range(), TextRange::new(1.into(), 9.into()));
    ///
    /// assert_eq!(node.text_range(), TextRange::new(0.into(), 11.into()));
    /// ```
    fn trimmed_range(&self) -> TextRange {
        let node = self.to_node();
        let tokens = node.lossy_tokens();
        let start = tokens
            .first()
            .map(|t| t.text_range().start())
            .unwrap_or_else(|| 0.into());
        let end = tokens
            .last()
            .map(|t| t.text_range().end())
            .unwrap_or_else(|| 0.into());

        TextRange::new(start, end)
    }

    /// Get the text of this node, not including leading or trailing whitespace
    ///
    /// # Examples
    /// ```
    /// use rslint_parser::{SyntaxNodeExt, parse_expr, TextRange};
    ///
    /// let node = parse_expr(" foo. bar  ", 0).syntax();
    ///
    /// assert_eq!(node.trimmed_text(), "foo. bar");
    /// ```
    fn trimmed_text(&self) -> SyntaxText {
        let trimmed = self.to_node().trimmed_range();
        let offset = self.to_node().text_range().start();
        self.to_node().text().slice(TextRange::new(
            trimmed.start() - offset,
            trimmed.end() - offset,
        ))
    }

    /// Get the directly adjacent previous token before the node.
    /// This could be whitespace (and most of the time it will be)
    /// therefore it is usually more useful to use the lossy version of this
    ///
    /// If the previous element is a node without tokens, the return value will be `None`
    fn prev_adjacent_token(&self) -> Option<SyntaxToken> {
        let node = self.to_node();
        let prev_element = node.prev_sibling_or_token()?;

        match prev_element {
            NodeOrToken::Node(node) => node.tokens().last().cloned(),
            NodeOrToken::Token(token) => Some(token),
        }
    }

    /// Get the directly adjacent next token after the node.
    /// This could be whitespace (and most of the time it will be)
    /// therefore it is usually more useful to use the lossy version of this
    ///
    /// If the next element is a node without tokens, the return value will be `None`
    fn next_adjacent_token(&self) -> Option<SyntaxToken> {
        let node = self.to_node();
        let next_element = node.next_sibling_or_token()?;

        match next_element {
            NodeOrToken::Node(node) => node.tokens().first().cloned(),
            NodeOrToken::Token(token) => Some(token),
        }
    }

    /// Get the directly adjacent previous (non whitespace) token before the node.
    ///
    /// # Examples
    /// ```
    /// use rslint_parser::{SyntaxNodeExt, parse_expr, ast::BinExpr, AstNode};
    ///
    /// let node = parse_expr("2 + 3 * 2", 0).syntax().to::<BinExpr>().rhs().unwrap().syntax().to_owned();
    ///
    /// assert_eq!(node.prev_adjacent_token_lossy().unwrap().text(), "2");
    /// ```
    fn prev_adjacent_token_lossy(&self) -> Option<SyntaxToken> {
        let node = self.to_node();
        let prev_element = node.prev_sibling_or_token()?;

        match prev_element {
            NodeOrToken::Node(node) => node.lossy_tokens().last().cloned(),
            NodeOrToken::Token(token) => {
                if !token.kind().is_trivia() {
                    Some(token)
                } else {
                    for element in node.siblings_with_tokens(Direction::Prev) {
                        match element {
                            NodeOrToken::Token(token) if !token.kind().is_trivia() => {
                                return Some(token)
                            }
                            NodeOrToken::Node(node) => return node.lossy_tokens().last().cloned(),
                            _ => {}
                        }
                    }
                    None
                }
            }
        }
    }

    /// Get the directly adjacent next (non whitespace) token after the node.
    ///
    /// # Examples
    /// ```
    /// use rslint_parser::{SyntaxNodeExt, parse_expr, ast::BinExpr, AstNode};
    ///
    /// let node = parse_expr("2 + 3 * 2", 0).syntax().to::<BinExpr>().lhs().unwrap().syntax().to_owned();
    ///
    /// assert_eq!(node.next_adjacent_token_lossy().unwrap().text(), "3");
    /// ```
    fn next_adjacent_token_lossy(&self) -> Option<SyntaxToken> {
        let node = self.to_node();
        let next_element = node.next_sibling_or_token()?;

        match next_element {
            NodeOrToken::Node(node) => node.lossy_tokens().first().cloned(),
            NodeOrToken::Token(token) => {
                if !token.kind().is_trivia() {
                    Some(token)
                } else {
                    for element in node.siblings_with_tokens(Direction::Next) {
                        match element {
                            NodeOrToken::Token(token) if !token.kind().is_trivia() => {
                                return Some(token)
                            }
                            NodeOrToken::Node(node) => return node.lossy_tokens().first().cloned(),
                            _ => {}
                        }
                    }
                    None
                }
            }
        }
    }

    /// Check whether this node's kind is contained in a token set.
    fn in_ts(&self, set: TokenSet) -> bool {
        set.contains(self.to_node().kind())
    }

    /// A human readable name for this node's kind. e.g.:
    /// `BREAK_STMT` => `Break statement`
    ///
    /// Returns a capitalized name without an underscore for anything not a statement. e.g.:
    /// `FN_DECL` => `Fn decl`
    fn readable_stmt_name(&self) -> String {
        let mut string = format!("{:?}", self.to_node().kind())
            .to_ascii_lowercase()
            .replace("_", " ");
        // Safety: the kind cannot produce an empty string and all kinds are ascii uppercase letters.
        unsafe {
            string.as_bytes_mut()[0] = string.as_bytes()[0] - 32;
        }

        if self.to_node().is::<ast::Stmt>() {
            string = string.replace("stmt", "statement");
        }

        string
    }

    /// Whether this node is an iteration statement.
    #[inline]
    fn is_loop(&self) -> bool {
        const ITERATION_STMT: [SyntaxKind; 5] = [
            SyntaxKind::FOR_IN_STMT,
            SyntaxKind::FOR_OF_STMT,
            SyntaxKind::FOR_STMT,
            SyntaxKind::WHILE_STMT,
            SyntaxKind::DO_WHILE_STMT,
        ];
        ITERATION_STMT.contains(&self.to_node().kind())
    }

    /// Go over the descendants of this node, at every descendant call `func`, and keep traversing
    /// the descendants of that node if the function's return is `true`. If the function returns false
    /// then stop traversing the descendants of that node go on to the next child.
    ///
    /// For example:
    /// ```ignore
    /// ROOT
    ///     CHILD // <-- Call `F` with CHILD, `F` returns `false` so go on to the next child...
    ///         SUBCHILD
    ///     CHILD // <-- Call `F` with CHILD, `F` return `true` so go on to the children of CHILD
    ///         SUBCHILD // <-- Same thing
    ///             SUBSUBCHILD
    ///     CHILD // Go on to the next child and do the same thing
    /// ```
    fn descendants_with<F>(&self, func: &mut F)
    where
        F: FnMut(&SyntaxNode) -> bool,
    {
        for node in self.to_node().children() {
            if func(&node) {
                node.descendants_with(func);
            }
        }
    }

    /// Separate all the lossy tokens of this node, then compare each token's text with the corresponding
    /// text in `tokens`. 
    fn structural_lossy_token_eq(&self, tokens: &[impl AsRef<str>]) -> bool {
        let node_tokens = self.to_node().lossy_tokens();
        if node_tokens.len() == tokens.len() {
            node_tokens
                .iter()
                .zip(tokens.iter())
                .all(|(l, r)| l.text() == r.as_ref())
        } else {
            false
        }
    }
}

impl SyntaxNodeExt for SyntaxNode {
    fn to_node(&self) -> &SyntaxNode {
        self
    }
}

pub trait SyntaxTokenExt {
    fn to_token(&self) -> &SyntaxToken;

    /// Convert a comment to a more detailed representation.
    fn comment(&self) -> Option<Comment> {
        if self.to_token().kind() != SyntaxKind::COMMENT {
            return None;
        }

        let token = self.to_token();
        let (kind, content) = match &token.text()[0..2] {
            "//" => (
                CommentKind::Inline,
                token
                    .text()
                    .get(2..)
                    .map(|x| x.to_string())
                    .unwrap_or_default(),
            ),
            "/*" if token.text().chars().nth(2) == Some('*') => {
                let len = token.text().len();
                let end = if token.text().get(len - 2..len) == Some("*/") {
                    len - 2
                } else {
                    len
                };
                (
                    CommentKind::JsDoc,
                    token
                        .text()
                        .get(3..end)
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                )
            }
            "/*" => {
                let len = token.text().len();
                let end = if token.text().get(len - 2..len) == Some("*/") {
                    len - 2
                } else {
                    len
                };
                (
                    CommentKind::JsDoc,
                    token
                        .text()
                        .get(3..end)
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                )
            }
            _ => return None,
        };
        Some(Comment { kind, content })
    }

    /// Check whether this token's kind is contained in a token set.
    fn in_ts(&self, set: TokenSet) -> bool {
        set.contains(self.to_token().kind())
    }
}

impl SyntaxTokenExt for SyntaxToken {
    fn to_token(&self) -> &SyntaxToken {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    pub kind: CommentKind,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommentKind {
    /// A block comment which starts with `/**`
    JsDoc,
    /// A block comment which starts with `/*`
    Multiline,
    /// An inline comment which starts with `//`
    Inline,
}

/// Concatenate tokens into a string
pub fn concat_tokens(tokens: &[SyntaxToken]) -> String {
    tokens
        .iter()
        .map(|token| token.text().to_string())
        .collect()
}