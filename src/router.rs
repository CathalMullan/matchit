use crate::tree::Node;
use crate::{InsertError, MatchError};

/// A zero-copy URL router.
///
/// See [the crate documentation](crate) for details.
#[derive(Clone, Debug)]
pub struct Router<T> {
    pub root: Node<T>,
}

impl<T> Default for Router<T> {
    fn default() -> Self {
        Self {
            root: Node::default(),
        }
    }
}

impl<T> Router<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, route: impl Into<String>, value: T) -> Result<(), InsertError> {
        self.root.insert(route.into(), value)
    }

    pub fn at<'path>(&self, path: &'path str) -> Result<Match<'_, 'path, &T>, MatchError> {
        match self.root.at(path.as_bytes()) {
            Ok((value, params)) => Ok(Match {
                // Safety: We only expose `&mut T` through `&mut self`
                value: unsafe { &*value.get() },
                params,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn at_mut<'path>(
        &mut self,
        path: &'path str,
    ) -> Result<Match<'_, 'path, &mut T>, MatchError> {
        match self.root.at(path.as_bytes()) {
            Ok((value, params)) => Ok(Match {
                // Safety: We have `&mut self`
                value: unsafe { &mut *value.get() },
                params,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn remove(&mut self, path: impl Into<String>) -> Option<T> {
        self.root.remove(path.into())
    }

    #[cfg(feature = "__test_helpers")]
    pub fn check_priorities(&self) -> Result<u32, (u32, u32)> {
        self.root.check_priorities()
    }
}

/// A successful match consisting of the registered value
/// and URL parameters, returned by [`Router::at`](Router::at).
#[derive(Debug)]
pub struct Match<'k, 'v, V> {
    /// The value stored under the matched node.
    pub value: V,

    /// The route parameters. See [parameters](crate#parameters) for more details.
    pub params: Vec<Param<'k, 'v>>,
}

/// A single URL parameter, consisting of a key and a value.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default, Copy, Clone)]
pub struct Param<'k, 'v> {
    pub key: &'k [u8],
    pub value: &'v [u8],
}
