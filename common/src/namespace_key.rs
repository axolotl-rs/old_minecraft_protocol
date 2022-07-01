use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamespaceKey<'nk> {
    pub namespace: &'nk str,
    pub key: &'nk str,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OwnedNamespaceKey {
    pub namespace: String,
    pub key: String,
}

impl OwnedNamespaceKey {
    pub fn new<S: Into<String>>(namespace: S, key: S) -> Self {
        Self {
            namespace: namespace.into(),
            key: key.into(),
        }
    }
}

impl Display for OwnedNamespaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.key)
    }
}

impl NamespaceKey<'static> {
    pub const fn new_const(namespace: &'static str, key: &'static str) -> Self {
        NamespaceKey {
            namespace,
            key,
        }
    }
}

impl<'nk> NamespaceKey<'nk> {
    pub fn new(namespace: &'nk str, key: &'nk str) -> Self {
        NamespaceKey {
            namespace,
            key,
        }
    }
}

impl<'nk> Display for NamespaceKey<'nk> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.key)
    }
}


pub trait AnyNamespaceKey{
    fn namespace(&self) -> &str;
    fn key(&self) -> &str;

    fn inner_ref(&self) -> (&str, &str);

    fn to_owned(self) -> OwnedNamespaceKey;
}

impl AnyNamespaceKey for NamespaceKey<'_> {
    fn namespace(&self) -> &str {
        self.namespace
    }
    fn key(&self) -> &str {
        self.key
    }

    fn inner_ref(&self) -> (&str, &str) {
        (self.namespace, self.key)
    }

    fn to_owned(self) -> OwnedNamespaceKey {
        OwnedNamespaceKey::new(self.namespace, self.key)
    }
}

impl AnyNamespaceKey for OwnedNamespaceKey {
    fn namespace(&self) -> &str {
        &self.namespace
    }
    fn key(&self) -> &str {
        &self.key
    }

    fn inner_ref(&self) -> (&str, &str) {
        (&self.namespace, &self.key)
    }

    fn to_owned(self) -> OwnedNamespaceKey {
        self
    }
}


#[cfg(test)]
pub mod test {
    use super::*;

    pub const TEST_NAMESPACE: NamespaceKey = NamespaceKey::new_const("minecraft", "stone");

    #[test]
    pub fn test() {
        assert_eq!(TEST_NAMESPACE.to_string(), "minecraft:stone");
    }
}