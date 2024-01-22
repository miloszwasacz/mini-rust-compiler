use std::fmt;

/// A span between two positions in a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    /// Creates a new `Span` between the given positions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mini_rust_compiler_components::token::{Position, Span};
    ///
    /// let start = Position::new_at(1, 1);
    /// let end = Position::new_at(2, 3);
    /// let span = Span::new(start, end);
    /// assert_eq!(span.start(), start);
    /// assert_eq!(span.end(), end);
    /// ```
    pub fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }

    /// Returns the start position of this `Span`.
    pub fn start(&self) -> Position {
        self.start
    }

    /// Returns the end position of this `Span`.
    pub fn end(&self) -> Position {
        self.end
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>-<{}>", self.start, self.end)
    }
}

/// A position in a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    /// Creates a new `Position` at line 1, column 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mini_rust_compiler_components::token::Position;
    ///
    /// let pos = Position::new();
    /// assert_eq!(pos.line(), 1);
    /// assert_eq!(pos.column(), 1);
    /// ```
    pub fn new() -> Position {
        Position::new_at(1, 1)
    }

    /// Creates a new `Position` at the given line and column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mini_rust_compiler_components::token::Position;
    ///
    /// let pos = Position::new_at(2, 3);
    /// assert_eq!(pos.line(), 2);
    /// assert_eq!(pos.column(), 3);
    /// ```
    pub fn new_at(line: usize, column: usize) -> Position {
        Position { line, column }
    }

    /// Returns the line of this `Position`.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Returns the column of this `Position`.
    pub fn column(&self) -> usize {
        self.column
    }

    /// Increments the line of this `Position` by 1 and sets the column to 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mini_rust_compiler_components::token::Position;
    ///
    /// let mut pos = Position::new_at(2, 3);
    /// pos.line_inc();
    /// assert_eq!(pos.line(), 3);
    /// assert_eq!(pos.column(), 1);
    /// ```
    pub fn line_inc(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    /// Increments the column of this `Position` by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mini_rust_compiler_components::token::Position;
    ///
    /// let mut pos = Position::new_at(2, 3);
    /// pos.col_inc();
    /// assert_eq!(pos.line(), 2);
    /// assert_eq!(pos.column(), 4);
    /// ```
    pub fn col_inc(&mut self) {
        self.column += 1;
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
