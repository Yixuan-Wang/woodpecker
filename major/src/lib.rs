use chrono::FixedOffset;
use once_cell::sync::Lazy;

pub mod hole;
pub(crate) mod util;

pub(crate) static DEFAULT_TIMEZONE_OFFSET: Lazy<FixedOffset> = Lazy::new(|| {
    let offset: i32 = option_env!("WOODPECKER_DEFAULT_TIMEZONE").map_or(8, |s| s.parse::<i32>().unwrap_or(8)) * 3600;
    FixedOffset::east(offset)
});

#[test]
fn test_utc() {
    use chrono::prelude::*;
    let now = Local::now().trunc_subsecs(0);
    dbg!(now);
}