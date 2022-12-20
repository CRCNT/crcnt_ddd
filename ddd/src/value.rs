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

impl From<&UtcDateTime> for Value {
  fn from(x: &UtcDateTime) -> Self {
    let naive = x.0.naive_utc();
    Value::from(naive)
  }
}

#[derive(Debug)]
pub struct CreateAtIr {
  ndt: NaiveDateTime,
}

impl ConvIr<CreateAt> for CreateAtIr {
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

impl FromValue for CreateAt {
  type Intermediate = CreateAtIr;
}
