use std::{fmt, slice};

/// A single URL parameter, consisting of a key and a value.
#[derive(PartialEq, Eq, Ord, PartialOrd, Default, Copy, Clone)]
struct Param<'k, 'v> {
    // Keys and values are stored as byte slices internally by the router
    // to avoid UTF8 checks when slicing, but UTF8 is still respected,
    // so these slices are valid strings.
    key: &'k [u8],
    value: &'v [u8],
}

impl<'k, 'v> Param<'k, 'v> {
    // Returns the parameter key as a string.
    fn key_str(&self) -> &'k str {
        std::str::from_utf8(self.key).unwrap()
    }

    // Returns the parameter value as a string.
    fn value_str(&self) -> &'v str {
        std::str::from_utf8(self.value).unwrap()
    }
}

/// A list of parameters returned by a route match.
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let mut router = matchit::Router::new();
/// # router.insert("/users/{id}", true).unwrap();
/// let matched = router.at("/users/1")?;
///
/// // Iterate through the keys and values.
/// for (key, value) in matched.params.iter() {
///     println!("key: {}, value: {}", key, value);
/// }
///
/// // Get a specific value by name.
/// let id = matched.params.get("id");
/// assert_eq!(id, Some("1"));
/// # Ok(())
/// # }
/// ```
#[derive(PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Params<'k, 'v>(Vec<Param<'k, 'v>>);

impl<'k, 'v> Params<'k, 'v> {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }

    /// Returns the number of parameters.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    // Truncates the parameter list to the given length.
    pub(crate) fn truncate(&mut self, n: usize) {
        self.0.truncate(n)
    }

    /// Returns the value of the first parameter registered under the given key.
    pub fn get(&self, key: impl AsRef<str>) -> Option<&'v str> {
        let key = key.as_ref().as_bytes();

        self.0
            .iter()
            .find(|param| param.key == key)
            .map(Param::value_str)
    }

    /// Returns an iterator over the parameters in the list.
    pub fn iter(&self) -> ParamsIter<'_, 'k, 'v> {
        ParamsIter::new(self)
    }

    /// Returns `true` if there are no parameters in the list.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Inserts a key value parameter pair into the list.
    pub(crate) fn push(&mut self, key: &'k [u8], value: &'v [u8]) {
        self.0.push(Param { key, value })
    }

    // Applies a transformation function to each key.
    pub(crate) fn for_each_key_mut(&mut self, f: impl Fn((usize, &mut &'k [u8]))) {
        self.0
            .iter_mut()
            .map(|param| &mut param.key)
            .enumerate()
            .for_each(f)
    }
}

impl fmt::Debug for Params<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// An iterator over the keys and values of a route's [parameters](crate::Params).
pub struct ParamsIter<'ps, 'k, 'v>(slice::Iter<'ps, Param<'k, 'v>>);

impl<'ps, 'k, 'v> ParamsIter<'ps, 'k, 'v> {
    fn new(params: &'ps Params<'k, 'v>) -> Self {
        Self(params.0.iter())
    }
}

impl<'ps, 'k, 'v> Iterator for ParamsIter<'ps, 'k, 'v> {
    type Item = (&'k str, &'v str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|p| (p.key_str(), p.value_str()))
    }
}

impl ExactSizeIterator for ParamsIter<'_, '_, '_> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_alloc() {
        let vec = vec![
            ("hello", "hello"),
            ("world", "world"),
            ("foo", "foo"),
            ("bar", "bar"),
            ("baz", "baz"),
        ];

        let mut params = Params::new();
        for (key, value) in vec.clone() {
            params.push(key.as_bytes(), value.as_bytes());
            assert_eq!(params.get(key), Some(value));
        }

        assert!(params.iter().eq(vec.clone()));
    }

    #[test]
    fn ignore_array_default() {
        let params = Params::new();
        assert!(params.get("").is_none());
    }
}
