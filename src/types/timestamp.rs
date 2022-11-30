use chrono::{DateTime, Utc, TimeZone, NaiveDateTime, LocalResult};
use chrono_tz::Tz;
use serde::Serialize;
use serde_json::{Value, json};
use std::hash::Hash;
use anyhow::{anyhow, Result};

/// [`Timestamp`] represents a timestamp *without* explicit timezone information.
/// The value is stored as number of milliseconds since epoch (1970-01-01), as this
/// is the format which is expected by elasticsearch
/// 
/// Although there is no original zone information stored with this value, you can 
/// create a [`Timestamp`] from timestamps originating from different timezones. 
#[derive(Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
pub struct Timestamp {
    ts: i64,
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_i64(self.ts)
    }
}

/// create a [`Timestamp`] from a [`DateTime`]
/// 
/// # Example
/// ```rust
/// use chrono_tz::US::Pacific;
/// use chrono::offset::TimeZone;
/// use ecs_types::types::Timestamp;
/// 
/// let pacific_time = Pacific.with_ymd_and_hms(1990, 5, 6, 12, 30, 45).single().unwrap();
/// let ts: Timestamp = pacific_time.into();
/// let tsi: i64 = ts.into();
/// assert_eq!(642022245000_i64, tsi); /* compare to Sun May 06 1990 19:30:45 GMT+0000 */
/// ```
impl<Tz> From<DateTime<Tz>> for Timestamp where Tz: TimeZone {
    fn from(d: DateTime<Tz>) -> Self {
        let ts = d.with_timezone(&Utc);
        Self {
            ts: ts.timestamp_millis(),
        }
    }
}

/// create a [`Timestamp`] from a unix timestamp together with a [`Tz`]
/// 
/// # Example
/// ```rust
/// use chrono_tz::Tz;
/// use chrono::offset::TimeZone;
/// use ecs_types::types::Timestamp;
/// 
/// let ts = Timestamp::try_from((641961045, &Tz::US__Pacific)).unwrap();
/// let tsi: i64 = ts.into();
/// //assert_eq!(642022245000_i64, tsi); /* compare to Sun May 06 1990 19:30:45 GMT+0000 */
/// ```
impl TryFrom<(i64, &Tz)> for Timestamp {
    type Error = anyhow::Error;

    fn try_from((unix_ts, src_tz): (i64, &Tz)) -> Result<Self, Self::Error> {
        let ts = match src_tz.from_local_datetime(&NaiveDateTime::from_timestamp_opt(unix_ts, 0).unwrap()) {
            LocalResult::None => {
                return Err(anyhow!("INVALID DATETIME"));
            }
            LocalResult::Single(t) => t,
            LocalResult::Ambiguous(t1, _t2) => t1,
        };
        Ok(
            Self {
                ts: ts.timestamp_millis(),
            }
        )
    }
}

impl From<Timestamp> for i64 {
    fn from(ts: Timestamp) -> Self {
        ts.ts
    }
}

impl From<&Timestamp> for Value {
    fn from(ts: &Timestamp) -> Self {
        json!(ts.ts)
    }
}

impl Timestamp {
    pub fn timestamp_millis(&self) -> i64 {
        self.ts
    }
}
