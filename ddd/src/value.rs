use {chrono::{DateTime,
              NaiveDateTime,
              Utc},
     crcnt_ddd_macros::DomainValue,
     mysql_async::Value,
     mysql_common::value::convert::{ConvIr,
                                    FromValue,
                                    FromValueError}};

/// CreateAt is a timestamp in milliseconds like it in Java
#[derive(Debug, Clone, DomainValue)]
pub struct CreateAt(UtcDateTime);

/// UpdateAt is a timestamp in milliseconds like it in Java
#[derive(Debug, Clone, DomainValue)]
pub struct UpdateAt(UtcDateTime);

/// Deleted is a logic deletion flag
#[derive(Debug, Clone, DomainValue)]
pub struct Deleted(bool);

/// Creator
#[derive(Debug, Clone, DomainValue)]
pub struct Creator(String);

/// Updater
#[derive(Debug, Clone, DomainValue)]
pub struct Updater(String);

#[derive(Debug, Clone)]
pub struct UtcDateTime(DateTime<Utc>);

impl UtcDateTime {
  pub fn now() -> Self { UtcDateTime(Utc::now()) }
}

impl CreateAt {
  pub fn now() -> Self { Self(UtcDateTime::now()) }

  pub fn timestamp(&self) -> i64 { *&self.0.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.0.timestamp_millis() }
}

impl UpdateAt {
  pub fn now() -> Self { Self(UtcDateTime::now()) }

  pub fn timestamp(&self) -> i64 { *&self.0.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.0.timestamp_millis() }
}

impl From<&UtcDateTime> for Value {
  fn from(x: &UtcDateTime) -> Self {
    let naive = x.0.naive_utc();
    Value::from(naive)
  }
}

#[derive(Debug)]
pub struct TimestampIr {
  ndt: NaiveDateTime,
}

#[derive(Debug)]
pub struct DeletedIr {
  b: bool,
}

#[derive(Debug)]
pub struct StrIr {
  bytes: Vec<u8>,
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

impl ConvIr<Deleted> for DeletedIr {
  fn new(v: Value) -> Result<Self, FromValueError> {
    let b = bool::from_value_opt(v)?;
    Ok(Self { b })
  }

  fn commit(self) -> Deleted { Deleted(self.b) }

  fn rollback(self) -> Value { Value::from(self.b) }
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

impl FromValue for CreateAt {
  type Intermediate = TimestampIr;
}
impl FromValue for UpdateAt {
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
