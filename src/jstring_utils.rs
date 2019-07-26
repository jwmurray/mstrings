pub trait JstringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl JstringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}
