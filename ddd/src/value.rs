use {chrono::{DateTime,
              Duration,
              NaiveDateTime,
              Utc},
     crcnt_ddd_macros::Domain,
     mysql_common::value::{convert::{ConvIr,
                                     FromValue,
                                     FromValueError},
                           Value},
     serde::{Deserialize,
             Serialize},
     std::{cmp::Ordering,
           ops},
     ulid::Ulid};

//<editor-fold desc="CreateAt Def">
/// CreateAt is a timestamp in milliseconds like it in Java
#[derive(Debug, Clone, Domain, Serialize, Deserialize)]
#[domain_commands(value)]
pub struct CreateAt(UtcDateTime);

impl CreateAt {
  pub fn now() -> Self { Self(UtcDateTime::now()) }

  pub fn from_utd_date_time(v: UtcDateTime) -> Self { Self(v) }

  pub fn timestamp(&self) -> i64 { *&self.0.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.0.timestamp_millis() }

  pub fn naive_date_time(&self) -> NaiveDateTime { self.0.0.naive_utc() }
}
//</editor-fold>

//<editor-fold desc="UpdateAt Def">
/// UpdateAt is a timestamp in milliseconds like it in Java
#[derive(Debug, Clone, Domain, Serialize, Deserialize)]
#[domain_commands(value)]
pub struct UpdateAt(UtcDateTime);

impl UpdateAt {
  pub fn now() -> Self { Self(UtcDateTime::now()) }

  pub fn from_utd_date_time(v: UtcDateTime) -> Self { Self(v) }

  pub fn timestamp(&self) -> i64 { *&self.0.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.0.timestamp_millis() }

  pub fn naive_date_time(&self) -> NaiveDateTime { self.0.0.naive_utc() }
}
//</editor-fold>

//<editor-fold desc="AvailableSince Def">
#[derive(Debug, Clone, Domain, Serialize, Deserialize)]
#[domain_commands(value)]
pub struct AvailableSince(UtcDateTime);

impl AvailableSince {
  pub fn now() -> Self { Self(UtcDateTime::now()) }

  pub fn timestamp(&self) -> i64 { *&self.0.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.0.timestamp_millis() }

  pub fn from_rfc3399<S: AsRef<str>>(s: S) -> Result<Self, String> { Ok(AvailableSince(UtcDateTime::from_rfc3339(s)?)) }

  pub fn is_available_now(&self) -> bool {
    let now = Utc::now();
    &self.0.0 <= &now
  }

  pub fn naive_date_time(&self) -> NaiveDateTime { self.0.0.naive_utc() }
}
//</editor-fold>

//<editor-fold desc="ExpiredSince Def">
#[derive(Debug, Clone, Domain, Serialize, Deserialize)]
#[domain_commands(value)]
pub struct ExpiredSince(UtcDateTime);

impl ExpiredSince {
  pub fn now() -> Self { Self(UtcDateTime::now()) }

  pub fn timestamp(&self) -> i64 { *&self.0.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.0.timestamp_millis() }

  pub fn from_rfc3399<S: AsRef<str>>(s: S) -> Result<Self, String> { Ok(ExpiredSince(UtcDateTime::from_rfc3339(s)?)) }

  pub fn is_expired_now(&self) -> bool {
    let now = Utc::now();
    &self.inner().0 <= &now
  }

  pub fn naive_date_time(&self) -> NaiveDateTime { self.0.0.naive_utc() }
}
//</editor-fold>

//<editor-fold desc="Basic String Value">
/// Creator
#[derive(Debug, Clone, Domain, Serialize, Deserialize)]
#[domain_commands(value)]
pub struct Creator(String);

/// Owner
#[derive(Debug, Clone, Domain, Serialize, Deserialize)]
#[domain_commands(value)]
pub struct Owner(String);

/// Owner
#[derive(Debug, Clone, Domain, Serialize, Deserialize)]
#[domain_commands(value)]
pub struct EntityId(String);
#[derive(Debug)]
pub struct EntityIdIr(pub EntityId);
/// Updater
#[derive(Debug, Clone, Domain, Serialize, Deserialize)]
#[domain_commands(value)]
pub struct Updater(String);
//</editor-fold>

//<editor-fold desc="UtcDateTime Def">
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtcDateTime(DateTime<Utc>);

impl UtcDateTime {
  pub fn now() -> Self { UtcDateTime(Utc::now()) }

  pub fn from_date_time(v: DateTime<Utc>) -> Self { Self(v) }

  pub fn from_rfc3339<S: AsRef<str>>(s: S) -> Result<Self, String> {
    let date_time = DateTime::parse_from_rfc3339(s.as_ref()).map_err(|e| format!("AvailableSince parse error: {}", e.to_string()))?;
    let date_time = date_time.with_timezone(&Utc);
    Ok(Self(date_time))
  }

  pub fn from_timestamp(timestamp: i64) -> Option<Self> {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0);
    if let Some(naive) = naive {
      let date_time = DateTime::<Utc>::from_utc(naive, Utc);
      Some(Self(date_time))
    } else {
      None
    }
  }

  pub fn from_timestamp_millis(timestamp: i64) -> Option<Self> {
    let naive = NaiveDateTime::from_timestamp_millis(timestamp);
    if let Some(naive) = naive {
      let date_time = DateTime::<Utc>::from_utc(naive, Utc);
      Some(Self(date_time))
    } else {
      None
    }
  }

  pub fn timestamp(&self) -> i64 { *&self.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.timestamp_millis() }

  pub fn naive_date_time(&self) -> NaiveDateTime { self.0.naive_utc() }
}
//</editor-fold>

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount(f64);
impl PartialEq for Amount {
  fn eq(&self, other: &Self) -> bool { (self.0 * 100.00) as u64 == (other.0 * 100.00) as u64 }
}
impl Ord for Amount {
  fn cmp(&self, other: &Self) -> Ordering {
    let a = (self.0 * 100.00) as u64;
    let b = (other.0 * 100.00) as u64;
    a.cmp(&b)
  }
}
impl PartialOrd for Amount {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    let a = (self.0 * 100.00) as u64;
    let b = (other.0 * 100.00) as u64;
    a.partial_cmp(&b)
  }
}
impl Eq for Amount {}
impl From<f64> for Amount {
  fn from(value: f64) -> Self { Amount((value * 100.00).round() / 100.00) }
}
impl Amount {
  pub fn new<T: Into<f64>>(v: T) -> Self {
    let x: f64 = v.into();
    From::from(x)
  }

  pub fn inner(&self) -> &f64 { &self.0 }

  pub fn into_inner(self) -> f64 { self.0 }
}
impl From<Amount> for Value {
  fn from(value: Amount) -> Self { Value::from(value.0) }
}
#[derive(Debug)]
pub struct F64Ir {
  v: f64,
}
impl ConvIr<Amount> for F64Ir {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let v = f64::from_value_opt(v)?;
    Ok(Self { v })
  }

  fn commit(self) -> Amount { Amount(self.v) }

  fn rollback(self) -> Value { Value::from(self.v) }
}
impl FromValue for Amount {
  type Intermediate = F64Ir;
}

//<editor-fold desc="ConvIr<T> for TimestampIr">
#[derive(Debug)]
pub struct TimestampIr {
  ndt: NaiveDateTime,
}
impl ConvIr<UtcDateTime> for TimestampIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let ndt = NaiveDateTime::from_value_opt(v)?;
    Ok(Self { ndt })
  }

  fn commit(self) -> UtcDateTime {
    let dt = DateTime::from_utc(self.ndt, Utc);
    UtcDateTime(dt)
  }

  fn rollback(self) -> Value { Value::from(self.ndt) }
}

impl ConvIr<CreateAt> for TimestampIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let ndt = NaiveDateTime::from_value_opt(v)?;
    Ok(Self { ndt })
  }

  fn commit(self) -> CreateAt {
    let dt = DateTime::from_utc(self.ndt, Utc);
    CreateAt(UtcDateTime(dt))
  }

  fn rollback(self) -> Value { Value::from(self.ndt) }
}

impl ConvIr<UpdateAt> for TimestampIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let ndt = NaiveDateTime::from_value_opt(v)?;
    Ok(Self { ndt })
  }

  fn commit(self) -> UpdateAt {
    let dt = DateTime::from_utc(self.ndt, Utc);
    UpdateAt(UtcDateTime(dt))
  }

  fn rollback(self) -> Value { Value::from(self.ndt) }
}

impl ConvIr<AvailableSince> for TimestampIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let ndt = NaiveDateTime::from_value_opt(v)?;
    Ok(Self { ndt })
  }

  fn commit(self) -> AvailableSince {
    let dt = DateTime::from_utc(self.ndt, Utc);
    AvailableSince(UtcDateTime(dt))
  }

  fn rollback(self) -> Value { Value::from(self.ndt) }
}

impl ConvIr<ExpiredSince> for TimestampIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let ndt = NaiveDateTime::from_value_opt(v)?;
    Ok(Self { ndt })
  }

  fn commit(self) -> ExpiredSince {
    let dt = DateTime::from_utc(self.ndt, Utc);
    ExpiredSince(UtcDateTime(dt))
  }

  fn rollback(self) -> Value { Value::from(self.ndt) }
}
//</editor-fold>

//<editor-fold desc="Deleted and ConvIr">
/// Deleted is a logic deletion flag
#[derive(Debug, Clone, Domain, Serialize, Deserialize)]
#[domain_commands(value)]
pub struct Deleted(bool);

#[derive(Debug)]
pub struct DeletedIr {
  b: bool,
}

impl ConvIr<Deleted> for DeletedIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let b = bool::from_value_opt(v)?;
    Ok(Self { b })
  }

  fn commit(self) -> Deleted { Deleted(self.b) }

  fn rollback(self) -> Value { Value::from(self.b) }
}
//</editor-fold>

//<editor-fold desc="ConvIr<T> for StrIr">
#[derive(Debug)]
pub struct StrIr {
  pub bytes: Vec<u8>,
}
impl ConvIr<Creator> for StrIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let bytes = Vec::<u8>::from_value_opt(v)?;
    Ok(StrIr { bytes })
  }

  fn commit(self) -> Creator {
    let creator = String::from_utf8_lossy(&self.bytes).to_string();
    Creator(creator)
  }

  fn rollback(self) -> Value { Value::from(self.bytes) }
}
impl ConvIr<Updater> for StrIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let bytes = Vec::<u8>::from_value_opt(v)?;
    Ok(StrIr { bytes })
  }

  fn commit(self) -> Updater {
    let updater = String::from_utf8_lossy(&self.bytes).to_string();
    Updater(updater)
  }

  fn rollback(self) -> Value { Value::from(self.bytes) }
}
impl ConvIr<Owner> for StrIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let bytes = Vec::<u8>::from_value_opt(v)?;
    Ok(StrIr { bytes })
  }

  fn commit(self) -> Owner {
    let owner = String::from_utf8_lossy(&self.bytes).to_string();
    Owner(owner)
  }

  fn rollback(self) -> Value { Value::from(self.bytes) }
}
impl ConvIr<EntityId> for StrIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let bytes = Vec::<u8>::from_value_opt(v)?;
    Ok(StrIr { bytes })
  }

  fn commit(self) -> EntityId {
    let id = String::from_utf8_lossy(&self.bytes).to_string();
    EntityId(id)
  }

  fn rollback(self) -> Value { Value::from(self.bytes) }
}
//</editor-fold>

//<editor-fold desc="impl FromValue">
impl FromValue for CreateAt {
  type Intermediate = TimestampIr;
}
impl FromValue for UpdateAt {
  type Intermediate = TimestampIr;
}
impl FromValue for AvailableSince {
  type Intermediate = TimestampIr;
}
impl FromValue for ExpiredSince {
  type Intermediate = TimestampIr;
}
impl FromValue for UtcDateTime {
  type Intermediate = TimestampIr;
}
impl FromValue for Deleted {
  type Intermediate = DeletedIr;
}
impl FromValue for Creator {
  type Intermediate = StrIr;
}
impl FromValue for Updater {
  type Intermediate = StrIr;
}
impl FromValue for Owner {
  type Intermediate = StrIr;
}
impl FromValue for EntityId {
  type Intermediate = StrIr;
}
//</editor-fold>

//<editor-fold desc="From<&T> for Value">
impl From<&UtcDateTime> for Value {
  fn from(x: &UtcDateTime) -> Self {
    let naive = x.0.naive_utc();
    Value::from(naive)
  }
}

//</editor-fold>

impl EntityId {
  pub fn new_with_prefix<T: AsRef<str>>(prefix: T) -> Self {
    let id = format!("{}{}", prefix.as_ref(), Ulid::new().to_string());
    EntityId::new(id)
  }
}

impl ops::Add<Duration> for UtcDateTime {
  type Output = UtcDateTime;

  fn add(self, rhs: Duration) -> Self::Output {
    let dt = self.0 + rhs;
    UtcDateTime(dt)
  }
}

impl ops::Sub for UtcDateTime {
  type Output = Duration;

  fn sub(self, rhs: Self) -> Self::Output { self.0 - rhs.0 }
}

impl PartialEq for UtcDateTime {
  fn eq(&self, other: &Self) -> bool { (&self.0).eq(&other.0) }
}
impl PartialOrd for UtcDateTime {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { (&self.0).partial_cmp(&other.0) }
}

#[cfg(test)]
mod test {
  use crate::value::Amount;

  #[test]
  fn test_amount() {
    let amt = Amount::new(0.64523);
    let amt2 = Amount::new(0.6463);
    let amt3 = Amount::new(0.6563);
    println!("amt = {:?}", amt);
    println!("{}", amt == amt2);
    println!("{}", amt < amt3);
  }
}
