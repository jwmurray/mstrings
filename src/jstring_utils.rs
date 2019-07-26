pub trait JstringUtils {
    fn msubstring(&self, start: usize, len: usize) -> Self;
    fn msnake_case(&self) -> String;
}

impl JstringUtils for String {
    fn msubstring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }

    fn msnake_case(&self) -> Self {
        self.chars().collect()
}
}
