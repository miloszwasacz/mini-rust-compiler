use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::Path;

/// A buffered reader that reads from a file.
pub struct FileReader<P> {
    path: P,
}

impl<P: AsRef<Path>> FileReader<P> {
    /// Creates a new `FileReader` that will read from the file at the given
    /// path.
    pub fn new(path: P) -> FileReader<P> {
        FileReader { path }
    }

    /// Tries to create a new `FileReaderIter` iterator from the file at the
    /// path provided in the constructor. If the file cannot be opened,
    /// an `io::Error` is returned.
    pub fn try_iter(&self) -> io::Result<FileReaderIter> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        Ok(FileReaderIter::new(reader))
    }
}

/// An iterator over the characters of a file.
pub struct FileReaderIter {
    /// A buffered reader that reads from a file.
    reader: BufReader<File>,
    /// A buffer that contains the last line read from the file
    /// (stored here only as an optimization to prevent unnecessary allocations).
    buffer_str: String,
    /// A buffer that contains the characters of the last line read from the file.
    buffer: Vec<char>,
    /// The index of the next character to be returned by the iterator.
    buffer_index: usize,
    /// The number of times the iterator has retried reading from the file.
    retries: u8,
}

impl FileReaderIter {
    /// Creates a new `FileReaderIter` iterator that uses the given `BufReader`
    /// as the source.
    pub fn new(reader: BufReader<File>) -> FileReaderIter {
        FileReaderIter {
            reader,
            buffer_str: String::new(),
            buffer: Vec::new(),
            buffer_index: 0,
            retries: 0,
        }
    }
}

impl Iterator for FileReaderIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        const RETRY_LIMIT: u8 = 10;

        // If there are still characters in the buffer, return the next one.
        match self.buffer.get(self.buffer_index) {
            Some(&c) => {
                self.buffer_index += 1;
                return Some(c);
            }
            None => {
                self.buffer_str.clear();
                self.buffer.clear();
                self.buffer_index = 1;
            }
        }

        // Otherwise, read the next line from the file.
        match self.reader.read_line(&mut self.buffer_str) {
            Ok(read_count) => {
                self.retries = 0;
                if read_count == 0 {
                    None
                } else {
                    self.buffer = self.buffer_str.chars().collect();
                    Some(self.buffer[0])
                }
            }
            Err(e) => match e.kind() {
                // Retry reading from the file if error is recoverable.
                ErrorKind::Interrupted if self.retries < RETRY_LIMIT => {
                    self.retries += 1;
                    self.next()
                }
                _ => None,
            },
        }
    }
}

//TODO Add tests
