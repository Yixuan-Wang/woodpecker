#[macro_export]
macro_rules! derive_set {
    ($the_field:ident, $the_type:tt, $the_entry:tt, $the_flag:tt, $the_page:tt, $the_set:tt) => {
        #[derive(Debug, Eq)]
        pub struct $the_entry {
            pub $the_field: $the_type,
            pub record: OffsetDateTime,
            pub flag: $the_flag,
        }
        
        #[derive(Debug, Default, PartialEq, Eq)]
        pub struct $the_set {
            set: HashSet<$the_entry>,
            flag: $the_flag,
        }
        
        impl $the_set {
            pub fn len(&self) -> usize {
                self.set.len()
            }
        
            pub fn is_empty(&self) -> bool {
                self.set.is_empty()
            }
        }
        
        impl Resource for $the_set {
            type Specifier = $the_flag;
        
            fn blank(flag: Self::Specifier) -> Self {
                $the_set {
                    set: HashSet::default(),
                    flag,
                }
            }
        }
        
        #[async_trait]
        impl ParseResource<$the_set> for $the_set {
            async fn parse(
                response: reqwest::Response,
                flag: $the_flag,
            ) -> Result<$the_set, ParseResourceError> {
                let page: $the_page = response.json().await?;
                Ok((page, flag).into())
            }
        }
        
        impl MergeResource<$the_set> for $the_set {
            fn merge(
                lhs: $the_set,
                rhs: $the_set,
                _flag: <$the_set as Resource>::Specifier,
            ) -> Result<$the_set, crate::common::MergeResourceError> {
                Ok(lhs | rhs)
            }
        }
        
        impl From<$the_page> for $the_set {
            fn from(page: $the_page) -> Self {
                (page, $the_flag::default()).into()
            }
        }
        
        impl IntoIterator for $the_set {
            type Item = $the_entry;
            type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;
        
            fn into_iter(self) -> Self::IntoIter {
                self.set.into_iter()
            }
        }
        
        impl BitOr for $the_set {
            type Output = Self;
        
            fn bitor(self, rhs: Self) -> Self::Output {
                assert_eq!(
                    self.flag, rhs.flag,
                    "`HashSet`s with {:?} and {:?} cannot be intersected",
                    self.flag, rhs.flag
                );
                let $the_set { mut set, flag } = self;
                let $the_set { set: set_rhs, .. } = rhs;
                set.extend(set_rhs);
                $the_set { set, flag }
            }
        }
        
        impl BitOrAssign for $the_set {
            fn bitor_assign(&mut self, rhs: Self) {
                assert_eq!(
                    self.flag, rhs.flag,
                    "`HashSet`s with {:?} and {:?} cannot be intersected",
                    self.flag, rhs.flag
                );
                let $the_set { set: set_rhs, .. } = rhs;
                self.set.extend(set_rhs);
            }
        }
    };
}
