use crate::Error;

mod words;

#[derive(Debug, PartialEq, Eq)]
pub struct Id<const N: usize> {
    words: [String; N],
}

impl<const N: usize> Id<N> {
    #[allow(clippy::new_without_default, reason = "the id is randomly generated")]
    pub fn new() -> Self {
        Self {
            words: std::array::from_fn(|_| words::get_random_word().to_string()),
        }
    }
}

impl<const N: usize> std::str::FromStr for Id<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            words: s
                .splitn(N, "-")
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| Error::InvalidId {
                    invalid_id: s.to_string(),
                    reason: "not enough words",
                })?,
        })
    }
}

impl<const N: usize> std::fmt::Display for Id<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.words.join("-"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "words-black-green";
        let expected = Id {
            words: [
                "words".to_string(),
                "black".to_string(),
                "green".to_string(),
            ],
        };
        let actual = input.parse().unwrap();

        assert_eq!(expected, actual);
    }
}
