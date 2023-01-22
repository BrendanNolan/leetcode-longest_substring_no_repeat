mod substring_search {
    struct CharSet {
        chars: [bool; 128],
    }

    impl CharSet {
        fn new() -> Self {
            Self {
                chars: [false; 128],
            }
        }

        fn contains(&self, c: u8) -> bool {
            self.chars[c as usize]
        }

        fn insert(&mut self, c: u8) {
            self.chars[c as usize] = true;
        }

        fn remove(&mut self, c: u8) {
            self.chars[c as usize] = false;
        }
    }

    #[derive(Copy, Clone)]
    struct Window<'a> {
        string: &'a [u8],
        begin: usize,
        end: usize,
    }

    impl<'a> Window<'a> {
        fn new(string: &'a [u8]) -> Self {
            Self {
                string,
                begin: 0,
                end: 0,
            }
        }

        fn len(&self) -> usize {
            self.end - self.begin
        }

        fn reached_end(&self) -> bool {
            self.end == self.string.len()
        }

        fn leftmost_char(&self) -> u8 {
            self.string[self.begin]
        }

        fn next_char(&self) -> u8 {
            self.string[self.end]
        }

        fn advance_end_point(&mut self, already_seen: &mut CharSet) -> Option<u8> {
            while self.end < self.string.len() && !already_seen.contains(self.next_char()) {
                already_seen.insert(self.next_char());
                self.end += 1;
            }
            if self.end == self.string.len() {
                None
            } else {
                Some(self.next_char())
            }
        }

        fn advance_begin_point(&mut self, advance_past: u8, already_seen: &mut CharSet) {
            while self.leftmost_char() != advance_past {
                already_seen.remove(self.leftmost_char());
                self.begin += 1;
            }
            already_seen.remove(self.leftmost_char());
            self.begin += 1;
        }

        fn slice(&self) -> &'a [u8] {
            &self.string[self.begin..self.end]
        }
    }

    fn find_longest_nonrepeating_substring_impl(s: &[u8]) -> &[u8] {
        let mut longest_window = Window::new(s);
        let mut sliding_window = Window::new(s);
        let mut already_seen = CharSet::new();
        while !sliding_window.reached_end() {
            let advance_past = sliding_window.advance_end_point(&mut already_seen);
            if sliding_window.len() > longest_window.len() {
                longest_window = sliding_window;
            }
            if let Some(duplicate_char) = advance_past {
                sliding_window.advance_begin_point(duplicate_char, &mut already_seen);
            }
        }
        longest_window.slice()
    }

    pub fn find_longest_nonrepeating_substring(s: &str) -> &str {
        let longest = find_longest_nonrepeating_substring_impl(s.as_bytes());
        std::str::from_utf8(longest).unwrap()
    }
}

fn main() {
    for s in &["acdaxnmklop", "abcabcbb", "bbbbb", "pwwkew", "dvdf"] {
        let longest_nonrepeating_substring =
            substring_search::find_longest_nonrepeating_substring(s);
        println!(
            "Longest non-repeating substring of \"{}\" is \"{}\", which has length {}",
            s,
            longest_nonrepeating_substring,
            longest_nonrepeating_substring.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::substring_search;

    #[test]
    fn test_0() {
        let longest = substring_search::find_longest_nonrepeating_substring("acdaxnmklop");
        assert_eq!(longest.len(), 10);
    }

    #[test]
    fn test_1() {
        let longest = substring_search::find_longest_nonrepeating_substring("abcabcbb");
        assert_eq!(longest.len(), 3);
    }

    #[test]
    fn test_2() {
        let longest = substring_search::find_longest_nonrepeating_substring("bbbbb");
        assert_eq!(longest.len(), 1);
    }

    #[test]
    fn test_3() {
        let longest = substring_search::find_longest_nonrepeating_substring("pwwkew");
        assert_eq!(longest.len(), 3);
    }

    #[test]
    fn test_4() {
        let longest = substring_search::find_longest_nonrepeating_substring("dvdf");
        assert_eq!(longest.len(), 3);
    }
}
