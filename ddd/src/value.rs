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

impl FromValue for CreateAt {
  type Intermediate = TimestampIr;
}
impl FromValue for UpdateAt {
  type Intermediate = TimestampIr;
}
impl FromValue for Deleted {
  type Intermediate = DeletedIr;
}
