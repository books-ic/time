use std::cmp::Ordering;
#[cfg(feature = "std")]
use std::time::SystemTime;
#[cfg(feature = "std")]
use time::UtcOffset;
use time::{
    error,
    ext::{NumericalDuration, NumericalStdDuration},
    Format, OffsetDateTime, Weekday,
};
use time_macros::{date, datetime, offset, time};

#[test]
#[cfg(feature = "std")]
fn now_utc() {
    assert!(OffsetDateTime::now_utc().year() >= 2019);
    assert_eq!(OffsetDateTime::now_utc().offset(), offset!("UTC"));
}

#[test]
#[cfg(feature = "std")]
fn now_local() {
    assert!(OffsetDateTime::now_local().year() >= 2019);
    assert_eq!(
        OffsetDateTime::now_local().offset(),
        UtcOffset::current_local_offset()
    );
}

#[test]
#[cfg(feature = "std")]
fn try_now_local() {
    assert!(OffsetDateTime::try_now_local().is_ok());
}

#[test]
fn to_offset() {
    assert_eq!(
        datetime!("2000-01-01 0:00")
            .assume_utc()
            .to_offset(offset!("-1"))
            .year(),
        1999,
    );

    let sydney = datetime!("2000-01-01 0:00").assume_offset(offset!("+11"));
    let new_york = sydney.to_offset(offset!("-5"));
    let los_angeles = sydney.to_offset(offset!("-8"));
    assert_eq!(sydney.hour(), 0);
    assert_eq!(sydney.day(), 1);
    assert_eq!(new_york.hour(), 8);
    assert_eq!(new_york.day(), 31);
    assert_eq!(los_angeles.hour(), 5);
    assert_eq!(los_angeles.day(), 31);
}

#[test]
fn unix_epoch() {
    assert_eq!(
        OffsetDateTime::unix_epoch(),
        datetime!("1970-01-01 0:00").assume_utc(),
    );
}

#[test]
fn from_unix_timestamp() {
    assert_eq!(
        OffsetDateTime::from_unix_timestamp(0),
        OffsetDateTime::unix_epoch(),
    );
    assert_eq!(
        OffsetDateTime::from_unix_timestamp(1_546_300_800),
        datetime!("2019-01-01 0:00").assume_utc(),
    );
}

#[test]
fn from_unix_timestamp_nanos() {
    assert_eq!(
        OffsetDateTime::from_unix_timestamp_nanos(0),
        OffsetDateTime::unix_epoch(),
    );
    assert_eq!(
        OffsetDateTime::from_unix_timestamp_nanos(1_546_300_800_000_000_000),
        datetime!("2019-01-01 0:00").assume_utc(),
    );
}

#[test]
fn offset() {
    assert_eq!(
        datetime!("2019-01-01 0:00").assume_utc().offset(),
        offset!("UTC"),
    );
    assert_eq!(
        datetime!("2019-01-01 0:00")
            .assume_offset(offset!("+1"))
            .offset(),
        offset!("+1"),
    );
    assert_eq!(
        datetime!("2019-01-01 0:00")
            .assume_utc()
            .to_offset(offset!("+1"))
            .offset(),
        offset!("+1"),
    );
}

#[test]
fn unix_timestamp() {
    assert_eq!(OffsetDateTime::unix_epoch().unix_timestamp(), 0);
    assert_eq!(
        OffsetDateTime::unix_epoch()
            .to_offset(offset!("+1"))
            .unix_timestamp(),
        0,
    );
    assert_eq!(
        datetime!("1970-01-01 0:00")
            .assume_offset(offset!("-1"))
            .unix_timestamp(),
        3_600,
    );
}

#[test]
fn unix_timestamp_nanos() {
    assert_eq!(
        datetime!("1970-01-01 0:00")
            .assume_utc()
            .unix_timestamp_nanos(),
        0,
    );
    assert_eq!(
        datetime!("1970-01-01 1:00")
            .assume_utc()
            .to_offset(offset!("-1"))
            .unix_timestamp_nanos(),
        3_600_000_000_000,
    );
}

#[test]
fn date() {
    assert_eq!(
        datetime!("2019-01-01 0:00").assume_utc().date(),
        date!("2019-01-01"),
    );
    assert_eq!(
        datetime!("2019-01-01 0:00")
            .assume_utc()
            .to_offset(offset!("-1"))
            .date(),
        date!("2018-12-31"),
    );
}

#[test]
fn time() {
    assert_eq!(
        datetime!("2019-01-01 0:00").assume_utc().time(),
        time!("0:00"),
    );
    assert_eq!(
        datetime!("2019-01-01 0:00")
            .assume_utc()
            .to_offset(offset!("-1"))
            .time(),
        time!("23:00"),
    );
}

#[test]
fn year() {
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().year(), 2019);
    assert_eq!(
        datetime!("2019-12-31 23:00")
            .assume_utc()
            .to_offset(offset!("+1"))
            .year(),
        2020,
    );
    assert_eq!(datetime!("2020-01-01 0:00").assume_utc().year(), 2020);
}

#[test]
fn month() {
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().month(), 1);
    assert_eq!(
        datetime!("2019-12-31 23:00")
            .assume_utc()
            .to_offset(offset!("+1"))
            .month(),
        1,
    );
}

#[test]
fn day() {
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().day(), 1);
    assert_eq!(
        datetime!("2019-12-31 23:00")
            .assume_utc()
            .to_offset(offset!("+1"))
            .day(),
        1,
    );
}

#[test]
fn month_day() {
    assert_eq!(
        datetime!("2019-01-01 0:00").assume_utc().month_day(),
        (1, 1),
    );
    assert_eq!(
        datetime!("2019-12-31 23:00")
            .assume_utc()
            .to_offset(offset!("+1"))
            .month_day(),
        (1, 1),
    );
}

#[test]
fn ordinal() {
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().ordinal(), 1);
    assert_eq!(
        datetime!("2019-12-31 23:00")
            .assume_utc()
            .to_offset(offset!("+1"))
            .ordinal(),
        1,
    );
}

#[test]
fn iso_year_week() {
    assert_eq!(
        datetime!("2019-01-01 0:00").assume_utc().iso_year_week(),
        (2019, 1)
    );
    assert_eq!(
        datetime!("2019-10-04 0:00").assume_utc().iso_year_week(),
        (2019, 40)
    );
    assert_eq!(
        datetime!("2020-01-01 0:00").assume_utc().iso_year_week(),
        (2020, 1)
    );
    assert_eq!(
        datetime!("2020-12-31 0:00").assume_utc().iso_year_week(),
        (2020, 53)
    );
    assert_eq!(
        datetime!("2021-01-01 0:00").assume_utc().iso_year_week(),
        (2020, 53)
    );
}

#[test]
fn week() {
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().week(), 1);
    assert_eq!(datetime!("2020-01-01 0:00").assume_utc().week(), 1);
    assert_eq!(datetime!("2020-12-31 0:00").assume_utc().week(), 53);
    assert_eq!(datetime!("2021-01-01 0:00").assume_utc().week(), 53);
}

#[test]
fn weekday() {
    use Weekday::*;
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().weekday(), Tuesday);
    assert_eq!(datetime!("2019-02-01 0:00").assume_utc().weekday(), Friday);
    assert_eq!(datetime!("2019-03-01 0:00").assume_utc().weekday(), Friday);
}

#[test]
fn hour() {
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().hour(), 0);
    assert_eq!(
        datetime!("2019-01-01 23:59:59")
            .assume_utc()
            .to_offset(offset!("-2"))
            .hour(),
        21,
    );
}

#[test]
fn minute() {
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().minute(), 0);
    assert_eq!(
        datetime!("2019-01-01 23:59:59")
            .assume_utc()
            .to_offset(offset!("+0:30"))
            .minute(),
        29,
    );
}

#[test]
fn second() {
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().second(), 0);
    assert_eq!(
        datetime!("2019-01-01 23:59:59")
            .assume_utc()
            .to_offset(offset!("+0:00:30"))
            .second(),
        29,
    );
}

#[test]
fn millisecond() {
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().millisecond(), 0);
    assert_eq!(
        datetime!("2019-01-01 23:59:59.999")
            .assume_utc()
            .millisecond(),
        999,
    );
}

#[test]
fn microsecond() {
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().microsecond(), 0);
    assert_eq!(
        datetime!("2019-01-01 23:59:59.999_999")
            .assume_utc()
            .microsecond(),
        999_999,
    );
}

#[test]
fn nanosecond() {
    assert_eq!(datetime!("2019-01-01 0:00").assume_utc().nanosecond(), 0);
    assert_eq!(
        datetime!("2019-01-01 23:59:59.999_999_999")
            .assume_utc()
            .nanosecond(),
        999_999_999,
    );
}

#[test]
fn format() {
    assert_eq!(
        datetime!("2019-01-02 0:00").assume_utc().format("%F %r %z"),
        "2019-01-02 12:00:00 am +0000",
    );
    assert_eq!(
        datetime!("2019-01-02 3:04:05.678_901_234")
            .assume_offset(offset!("+6:07"))
            .format(Format::Rfc3339),
        "2019-01-02T03:04:05+06:07"
    );
}

#[test]
fn parse() {
    assert_eq!(
        OffsetDateTime::parse("2019-01-02 00:00:00 +0000", "%F %T %z"),
        Ok(datetime!("2019-01-02 0:00").assume_utc()),
    );
    assert_eq!(
        OffsetDateTime::parse("2019-002 23:59:59 +0000", "%Y-%j %T %z"),
        Ok(datetime!("2019-002 23:59:59").assume_utc())
    );
    assert_eq!(
        OffsetDateTime::parse("2019-W01-3 12:00:00 pm +0000", "%G-W%V-%u %r %z"),
        Ok(datetime!("2019-002 12:00").assume_utc())
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02 03:04:05 +0600", "%F %T %z"),
        Ok(datetime!("2019-01-02 3:04:05").assume_offset(offset!("+6")))
    );
    assert_eq!(
        OffsetDateTime::parse("2020-09-08T08:44:31+02:30", Format::Rfc3339),
        Ok(datetime!("2020-09-08 08:44:31").assume_offset(offset!("+02:30")))
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02T03:04:05.678901234+05:06", Format::Rfc3339),
        Ok(datetime!("2019-01-02 3:04:05.678_901_234").assume_offset(offset!("+5:06")))
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02T03:04:05.678901234Z", Format::Rfc3339),
        Ok(datetime!("2019-01-02 3:04:05.678_901_234").assume_utc())
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02T03:04:05/", Format::Rfc3339),
        Err(error::Parse::UnexpectedCharacter {
            actual: '/',
            expected: '+'
        })
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02T03:04:05", Format::Rfc3339),
        Err(error::Parse::UnexpectedEndOfString)
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02T03:04:05.", Format::Rfc3339),
        Err(error::Parse::InvalidNanosecond)
    );
}

#[test]
fn partial_eq() {
    assert_eq!(
        datetime!("2000-01-01 0:00")
            .assume_utc()
            .to_offset(offset!("-1")),
        datetime!("2000-01-01 0:00").assume_utc(),
    );
}

#[test]
fn partial_ord() {
    let t1 = datetime!("2019-01-01 0:00").assume_utc();
    let t2 = datetime!("2019-01-01 0:00")
        .assume_utc()
        .to_offset(offset!("-1"));
    assert_eq!(t1.partial_cmp(&t2), Some(Ordering::Equal));
}

#[test]
fn ord() {
    let t1 = datetime!("2019-01-01 0:00").assume_utc();
    let t2 = datetime!("2019-01-01 0:00")
        .assume_utc()
        .to_offset(offset!("-1"));
    assert_eq!(t1, t2);

    let t1 = datetime!("2019-01-01 0:00").assume_utc();
    let t2 = datetime!("2019-01-01 0:00:00.000_000_001").assume_utc();
    assert!(t2 > t1);
}

#[test]
#[cfg(feature = "std")]
fn hash() {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    assert_eq!(
        {
            let mut hasher = DefaultHasher::new();
            datetime!("2019-01-01 0:00").assume_utc().hash(&mut hasher);
            hasher.finish()
        },
        {
            let mut hasher = DefaultHasher::new();
            datetime!("2019-01-01 0:00")
                .assume_utc()
                .to_offset(offset!("-1"))
                .hash(&mut hasher);
            hasher.finish()
        }
    );

    // Ensure that a `PrimitiveDateTime` and `OffsetDateTime` don't collide,
    // even if the UTC time is the same.
    assert_ne!(
        {
            let mut hasher = DefaultHasher::new();
            datetime!("2019-01-01 0:00").hash(&mut hasher);
            hasher.finish()
        },
        {
            let mut hasher = DefaultHasher::new();
            datetime!("2019-01-01 0:00").assume_utc().hash(&mut hasher);
            hasher.finish()
        }
    );
}

#[test]
fn add_duration() {
    assert_eq!(
        datetime!("2019-01-01 0:00").assume_utc() + 5.days(),
        datetime!("2019-01-06 0:00").assume_utc(),
    );
    assert_eq!(
        datetime!("2019-12-31 0:00").assume_utc() + 1.days(),
        datetime!("2020-01-01 0:00").assume_utc(),
    );
    assert_eq!(
        datetime!("2019-12-31 23:59:59").assume_utc() + 2.seconds(),
        datetime!("2020-01-01 0:00:01").assume_utc(),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00:01").assume_utc() + (-2).seconds(),
        datetime!("2019-12-31 23:59:59").assume_utc(),
    );
    assert_eq!(
        datetime!("1999-12-31 23:00").assume_utc() + 1.hours(),
        datetime!("2000-01-01 0:00").assume_utc(),
    );
}

#[test]
fn add_std_duration() {
    assert_eq!(
        datetime!("2019-01-01 0:00").assume_utc() + 5.std_days(),
        datetime!("2019-01-06 0:00").assume_utc(),
    );
    assert_eq!(
        datetime!("2019-12-31 0:00").assume_utc() + 1.std_days(),
        datetime!("2020-01-01 0:00").assume_utc(),
    );
    assert_eq!(
        datetime!("2019-12-31 23:59:59").assume_utc() + 2.std_seconds(),
        datetime!("2020-01-01 0:00:01").assume_utc(),
    );
}

#[test]
fn add_assign_duration() {
    let mut ny19 = datetime!("2019-01-01 0:00").assume_utc();
    ny19 += 5.days();
    assert_eq!(ny19, datetime!("2019-01-06 0:00").assume_utc());

    let mut nye20 = datetime!("2019-12-31 0:00").assume_utc();
    nye20 += 1.days();
    assert_eq!(nye20, datetime!("2020-01-01 0:00").assume_utc());

    let mut nye20t = datetime!("2019-12-31 23:59:59").assume_utc();
    nye20t += 2.seconds();
    assert_eq!(nye20t, datetime!("2020-01-01 0:00:01").assume_utc());

    let mut ny20t = datetime!("2020-01-01 0:00:01").assume_utc();
    ny20t += (-2).seconds();
    assert_eq!(ny20t, datetime!("2019-12-31 23:59:59").assume_utc());
}

#[test]
fn add_assign_std_duration() {
    let mut ny19 = datetime!("2019-01-01 0:00").assume_utc();
    ny19 += 5.std_days();
    assert_eq!(ny19, datetime!("2019-01-06 0:00").assume_utc());

    let mut nye20 = datetime!("2019-12-31 0:00").assume_utc();
    nye20 += 1.std_days();
    assert_eq!(nye20, datetime!("2020-01-01 0:00").assume_utc());

    let mut nye20t = datetime!("2019-12-31 23:59:59").assume_utc();
    nye20t += 2.std_seconds();
    assert_eq!(nye20t, datetime!("2020-01-01 0:00:01").assume_utc());
}

#[test]
fn sub_duration() {
    assert_eq!(
        datetime!("2019-01-06 0:00").assume_utc() - 5.days(),
        datetime!("2019-01-01 0:00").assume_utc(),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00").assume_utc() - 1.days(),
        datetime!("2019-12-31 0:00").assume_utc(),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00:01").assume_utc() - 2.seconds(),
        datetime!("2019-12-31 23:59:59").assume_utc(),
    );
    assert_eq!(
        datetime!("2019-12-31 23:59:59").assume_utc() - (-2).seconds(),
        datetime!("2020-01-01 0:00:01").assume_utc(),
    );
    assert_eq!(
        datetime!("1999-12-31 23:00").assume_utc() - (-1).hours(),
        datetime!("2000-01-01 0:00").assume_utc(),
    );
}

#[test]
fn sub_std_duration() {
    assert_eq!(
        datetime!("2019-01-06 0:00").assume_utc() - 5.std_days(),
        datetime!("2019-01-01 0:00").assume_utc(),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00").assume_utc() - 1.std_days(),
        datetime!("2019-12-31 0:00").assume_utc(),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00:01").assume_utc() - 2.std_seconds(),
        datetime!("2019-12-31 23:59:59").assume_utc(),
    );
}

#[test]
fn sub_assign_duration() {
    let mut ny19 = datetime!("2019-01-06 0:00").assume_utc();
    ny19 -= 5.days();
    assert_eq!(ny19, datetime!("2019-01-01 0:00").assume_utc());

    let mut ny20 = datetime!("2020-01-01 0:00").assume_utc();
    ny20 -= 1.days();
    assert_eq!(ny20, datetime!("2019-12-31 0:00").assume_utc());

    let mut ny20t = datetime!("2020-01-01 0:00:01").assume_utc();
    ny20t -= 2.seconds();
    assert_eq!(ny20t, datetime!("2019-12-31 23:59:59").assume_utc());

    let mut nye20t = datetime!("2019-12-31 23:59:59").assume_utc();
    nye20t -= (-2).seconds();
    assert_eq!(nye20t, datetime!("2020-01-01 0:00:01").assume_utc());
}

#[test]
fn sub_assign_std_duration() {
    let mut ny19 = datetime!("2019-01-06 0:00").assume_utc();
    ny19 -= 5.std_days();
    assert_eq!(ny19, datetime!("2019-01-01 0:00").assume_utc());

    let mut ny20 = datetime!("2020-01-01 0:00").assume_utc();
    ny20 -= 1.std_days();
    assert_eq!(ny20, datetime!("2019-12-31 0:00").assume_utc());

    let mut ny20t = datetime!("2020-01-01 0:00:01").assume_utc();
    ny20t -= 2.std_seconds();
    assert_eq!(ny20t, datetime!("2019-12-31 23:59:59").assume_utc());
}

#[test]
#[cfg(feature = "std")]
fn std_add_duration() {
    assert_eq!(
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc()) + 0.seconds(),
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc()),
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc()) + 5.days(),
        SystemTime::from(datetime!("2019-01-06 0:00").assume_utc()),
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-12-31 0:00").assume_utc()) + 1.days(),
        SystemTime::from(datetime!("2020-01-01 0:00").assume_utc()),
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-12-31 23:59:59").assume_utc()) + 2.seconds(),
        SystemTime::from(datetime!("2020-01-01 0:00:01").assume_utc()),
    );
    assert_eq!(
        SystemTime::from(datetime!("2020-01-01 0:00:01").assume_utc()) + (-2).seconds(),
        SystemTime::from(datetime!("2019-12-31 23:59:59").assume_utc()),
    );
}

#[test]
#[cfg(feature = "std")]
fn std_add_assign_duration() {
    let mut ny19 = SystemTime::from(datetime!("2019-01-01 0:00").assume_utc());
    ny19 += 5.days();
    assert_eq!(ny19, datetime!("2019-01-06 0:00").assume_utc());

    let mut nye20 = SystemTime::from(datetime!("2019-12-31 0:00").assume_utc());
    nye20 += 1.days();
    assert_eq!(nye20, datetime!("2020-01-01 0:00").assume_utc());

    let mut nye20t = SystemTime::from(datetime!("2019-12-31 23:59:59").assume_utc());
    nye20t += 2.seconds();
    assert_eq!(nye20t, datetime!("2020-01-01 0:00:01").assume_utc());

    let mut ny20t = SystemTime::from(datetime!("2020-01-01 0:00:01").assume_utc());
    ny20t += (-2).seconds();
    assert_eq!(ny20t, datetime!("2019-12-31 23:59:59").assume_utc());
}

#[test]
#[cfg(feature = "std")]
fn std_sub_duration() {
    assert_eq!(
        SystemTime::from(datetime!("2019-01-06 0:00").assume_utc()) - 5.days(),
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc()),
    );
    assert_eq!(
        SystemTime::from(datetime!("2020-01-01 0:00").assume_utc()) - 1.days(),
        SystemTime::from(datetime!("2019-12-31 0:00").assume_utc()),
    );
    assert_eq!(
        SystemTime::from(datetime!("2020-01-01 0:00:01").assume_utc()) - 2.seconds(),
        SystemTime::from(datetime!("2019-12-31 23:59:59").assume_utc()),
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-12-31 23:59:59").assume_utc()) - (-2).seconds(),
        SystemTime::from(datetime!("2020-01-01 0:00:01").assume_utc()),
    );
}

#[test]
#[cfg(feature = "std")]
fn std_sub_assign_duration() {
    let mut ny19 = SystemTime::from(datetime!("2019-01-06 0:00").assume_utc());
    ny19 -= 5.days();
    assert_eq!(ny19, datetime!("2019-01-01 0:00").assume_utc());

    let mut ny20 = SystemTime::from(datetime!("2020-01-01 0:00").assume_utc());
    ny20 -= 1.days();
    assert_eq!(ny20, datetime!("2019-12-31 0:00").assume_utc());

    let mut ny20t = SystemTime::from(datetime!("2020-01-01 0:00:01").assume_utc());
    ny20t -= 2.seconds();
    assert_eq!(ny20t, datetime!("2019-12-31 23:59:59").assume_utc());

    let mut nye20t = SystemTime::from(datetime!("2019-12-31 23:59:59").assume_utc());
    nye20t -= (-2).seconds();
    assert_eq!(nye20t, datetime!("2020-01-01 0:00:01").assume_utc());
}

#[test]
fn sub_self() {
    assert_eq!(
        datetime!("2019-01-02 0:00").assume_utc() - datetime!("2019-01-01 0:00").assume_utc(),
        1.days(),
    );
    assert_eq!(
        datetime!("2019-01-01 0:00").assume_utc() - datetime!("2019-01-02 0:00").assume_utc(),
        (-1).days(),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00").assume_utc() - datetime!("2019-12-31 0:00").assume_utc(),
        1.days(),
    );
    assert_eq!(
        datetime!("2019-12-31 0:00").assume_utc() - datetime!("2020-01-01 0:00").assume_utc(),
        (-1).days(),
    );
}

#[test]
#[cfg(feature = "std")]
fn std_sub() {
    assert_eq!(
        SystemTime::from(datetime!("2019-01-02 0:00").assume_utc())
            - datetime!("2019-01-01 0:00").assume_utc(),
        1.days()
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
            - datetime!("2019-01-02 0:00").assume_utc(),
        (-1).days()
    );
    assert_eq!(
        SystemTime::from(datetime!("2020-01-01 0:00").assume_utc())
            - datetime!("2019-12-31 0:00").assume_utc(),
        1.days()
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-12-31 0:00").assume_utc())
            - datetime!("2020-01-01 0:00").assume_utc(),
        (-1).days()
    );
}

#[test]
#[cfg(feature = "std")]
fn sub_std() {
    assert_eq!(
        datetime!("2019-01-02 0:00").assume_utc()
            - SystemTime::from(datetime!("2019-01-01 0:00").assume_utc()),
        1.days()
    );
    assert_eq!(
        datetime!("2019-01-01 0:00").assume_utc()
            - SystemTime::from(datetime!("2019-01-02 0:00").assume_utc()),
        (-1).days()
    );
    assert_eq!(
        datetime!("2020-01-01 0:00").assume_utc()
            - SystemTime::from(datetime!("2019-12-31 0:00").assume_utc()),
        1.days()
    );
    assert_eq!(
        datetime!("2019-12-31 0:00").assume_utc()
            - SystemTime::from(datetime!("2020-01-01 0:00").assume_utc()),
        (-1).days()
    );
}

#[test]
#[cfg(feature = "std")]
fn eq_std() {
    let now_datetime = OffsetDateTime::now_utc();
    let now_systemtime = SystemTime::from(now_datetime);
    assert_eq!(now_datetime, now_systemtime);
}

#[test]
#[cfg(feature = "std")]
fn std_eq() {
    let now_datetime = OffsetDateTime::now_utc();
    let now_systemtime = SystemTime::from(now_datetime);
    assert_eq!(now_datetime, now_systemtime);
}

#[test]
#[cfg(feature = "std")]
fn ord_std() {
    assert_eq!(
        datetime!("2019-01-01 0:00").assume_utc(),
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
    );
    assert!(
        datetime!("2019-01-01 0:00").assume_utc()
            < SystemTime::from(datetime!("2020-01-01 0:00").assume_utc())
    );
    assert!(
        datetime!("2019-01-01 0:00").assume_utc()
            < SystemTime::from(datetime!("2019-02-01 0:00").assume_utc())
    );
    assert!(
        datetime!("2019-01-01 0:00").assume_utc()
            < SystemTime::from(datetime!("2019-01-02 0:00").assume_utc())
    );
    assert!(
        datetime!("2019-01-01 0:00").assume_utc()
            < SystemTime::from(datetime!("2019-01-01 1:00:00").assume_utc())
    );
    assert!(
        datetime!("2019-01-01 0:00").assume_utc()
            < SystemTime::from(datetime!("2019-01-01 0:01:00").assume_utc())
    );
    assert!(
        datetime!("2019-01-01 0:00").assume_utc()
            < SystemTime::from(datetime!("2019-01-01 0:00:01").assume_utc())
    );
    assert!(
        datetime!("2019-01-01 0:00").assume_utc()
            < SystemTime::from(datetime!("2019-01-01 0:00:00.001").assume_utc())
    );
    assert!(
        datetime!("2020-01-01 0:00").assume_utc()
            > SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
    );
    assert!(
        datetime!("2019-02-01 0:00").assume_utc()
            > SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
    );
    assert!(
        datetime!("2019-01-02 0:00").assume_utc()
            > SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
    );
    assert!(
        datetime!("2019-01-01 1:00:00").assume_utc()
            > SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
    );
    assert!(
        datetime!("2019-01-01 0:01:00").assume_utc()
            > SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
    );
    assert!(
        datetime!("2019-01-01 0:00:01").assume_utc()
            > SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
    );
    assert!(
        datetime!("2019-01-01 0:00:00.000_000_001").assume_utc()
            > SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
    );
}

#[test]
#[cfg(feature = "std")]
fn std_ord() {
    assert_eq!(
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc()),
        datetime!("2019-01-01 0:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
            < datetime!("2020-01-01 0:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
            < datetime!("2019-02-01 0:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
            < datetime!("2019-01-02 0:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
            < datetime!("2019-01-01 1:00:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
            < datetime!("2019-01-01 0:01:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
            < datetime!("2019-01-01 0:00:01").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00").assume_utc())
            < datetime!("2019-01-01 0:00:00.000_000_001").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2020-01-01 0:00").assume_utc())
            > datetime!("2019-01-01 0:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-02-01 0:00").assume_utc())
            > datetime!("2019-01-01 0:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-02 0:00").assume_utc())
            > datetime!("2019-01-01 0:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 1:00:00").assume_utc())
            > datetime!("2019-01-01 0:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:01:00").assume_utc())
            > datetime!("2019-01-01 0:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00:01").assume_utc())
            > datetime!("2019-01-01 0:00").assume_utc()
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00:00.001").assume_utc())
            > datetime!("2019-01-01 0:00").assume_utc()
    );
}

#[test]
#[cfg(feature = "std")]
fn from_std() {
    assert_eq!(
        OffsetDateTime::from(SystemTime::UNIX_EPOCH),
        OffsetDateTime::unix_epoch()
    );
    assert_eq!(
        OffsetDateTime::from(SystemTime::UNIX_EPOCH - 1.std_days()),
        OffsetDateTime::unix_epoch() - 1.days()
    );
    assert_eq!(
        OffsetDateTime::from(SystemTime::UNIX_EPOCH + 1.std_days()),
        OffsetDateTime::unix_epoch() + 1.days()
    );
}

#[test]
#[cfg(feature = "std")]
fn to_std() {
    assert_eq!(
        SystemTime::from(OffsetDateTime::unix_epoch()),
        SystemTime::UNIX_EPOCH
    );
    assert_eq!(
        SystemTime::from(OffsetDateTime::unix_epoch() + 1.days()),
        SystemTime::UNIX_EPOCH + 1.std_days()
    );
    assert_eq!(
        SystemTime::from(OffsetDateTime::unix_epoch() - 1.days()),
        SystemTime::UNIX_EPOCH - 1.std_days()
    );
}

#[test]
#[cfg(feature = "std")]
fn display() {
    assert_eq!(
        datetime!("1970-01-01 0:00").assume_utc().to_string(),
        String::from("1970-01-01 0:00 +0")
    );
}
