use {chrono::{DateTime,
              NaiveDateTime,
              Utc},
     crcnt_ddd_macros::Domain,
     mysql_common::value::{convert::{ConvIr,
                                     FromValue,
                                     FromValueError},
                           Value}};

//<editor-fold desc="CreateAt Def">
/// CreateAt is a timestamp in milliseconds like it in Java
#[derive(Debug, Clone, Domain)]
#[domain_commands(value)]
pub struct CreateAt(UtcDateTime);

impl CreateAt {
  pub fn now() -> Self { Self(UtcDateTime::now()) }

  pub fn timestamp(&self) -> i64 { *&self.0.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.0.timestamp_millis() }

  pub fn naive_date_time(&self) -> NaiveDateTime { self.0.0.naive_utc() }
}
//</editor-fold>

//<editor-fold desc="UpdateAt Def">
/// UpdateAt is a timestamp in milliseconds like it in Java
#[derive(Debug, Clone, Domain)]
#[domain_commands(value)]
pub struct UpdateAt(UtcDateTime);

impl UpdateAt {
  pub fn now() -> Self { Self(UtcDateTime::now()) }

  pub fn timestamp(&self) -> i64 { *&self.0.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.0.timestamp_millis() }

  pub fn naive_date_time(&self) -> NaiveDateTime { self.0.0.naive_utc() }
}
//</editor-fold>

//<editor-fold desc="AvailableSince Def">
#[derive(Debug, Clone, Domain)]
#[domain_commands(value)]
pub struct AvailableSince(UtcDateTime);

impl AvailableSince {
  pub fn now() -> Self { Self(UtcDateTime::now()) }

  pub fn timestamp(&self) -> i64 { *&self.0.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.0.timestamp_millis() }

  pub fn from_rfc3399(s: &str) -> Result<Self, String> {
    let date_time = DateTime::parse_from_rfc3339(s).map_err(|e| format!("AvailableSince parse error: {}", e.to_string()))?;
    let expired = date_time.with_timezone(&Utc);
    Ok(AvailableSince(UtcDateTime(expired)))
  }

  pub fn is_available_now(&self) -> bool {
    let now = Utc::now();
    &self.0.0 <= &now
  }

  pub fn naive_date_time(&self) -> NaiveDateTime { self.0.0.naive_utc() }
}
//</editor-fold>

//<editor-fold desc="ExpiredSince Def">
#[derive(Debug, Clone, Domain)]
#[domain_commands(value)]
pub struct ExpiredSince(UtcDateTime);

impl ExpiredSince {
  pub fn now() -> Self { Self(UtcDateTime::now()) }

  pub fn timestamp(&self) -> i64 { *&self.0.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.0.timestamp_millis() }

  pub fn from_rfc3399(s: &str) -> Result<Self, String> {
    let date_time = DateTime::parse_from_rfc3339(s).map_err(|e| format!("ExpiredSince parse error: {}", e.to_string()))?;
    let expired = date_time.with_timezone(&Utc);
    Ok(ExpiredSince(UtcDateTime(expired)))
  }

  pub fn is_expired_now(&self) -> bool {
    let now = Utc::now();
    &self.inner().0 <= &now
  }

  pub fn naive_date_time(&self) -> NaiveDateTime { self.0.0.naive_utc() }
}
//</editor-fold>

//<editor-fold desc="Basic String Value">
/// Creator
#[derive(Debug, Clone, Domain)]
#[domain_commands(value)]
pub struct Creator(String);

/// Owner
#[derive(Debug, Clone, Domain)]
#[domain_commands(value)]
pub struct Owner(String);

/// Owner
#[derive(Debug, Clone, Domain)]
#[domain_commands(value)]
pub struct EntityId(String);

/// Updater
#[derive(Debug, Clone, Domain)]
#[domain_commands(value)]
pub struct Updater(String);
//</editor-fold>

//<editor-fold desc="UtcDateTime Def">
#[derive(Debug, Clone)]
pub struct UtcDateTime(DateTime<Utc>);

impl UtcDateTime {
  pub fn now() -> Self { UtcDateTime(Utc::now()) }

  pub fn timestamp(&self) -> i64 { *&self.0.timestamp() }

  pub fn timestamp_millis(&self) -> i64 { *&self.0.timestamp_millis() }

  pub fn naive_date_time(&self) -> NaiveDateTime { self.0.naive_utc() }
}
//</editor-fold>

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
#[derive(Debug, Clone, Domain)]
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
impl From<&CreateAt> for Value {
  fn from(x: &CreateAt) -> Self { Value::from(x.naive_date_time()) }
}

impl From<&UpdateAt> for Value {
  fn from(x: &UpdateAt) -> Self { Value::from(x.naive_date_time()) }
}

impl From<&Deleted> for Value {
  fn from(x: &Deleted) -> Self { Value::from(x.0) }
}

impl From<&Creator> for Value {
  fn from(x: &Creator) -> Self { Value::from(x.inner()) }
}

impl From<&Updater> for Value {
  fn from(x: &Updater) -> Self { Value::from(x.inner()) }
}
impl From<&Owner> for Value {
  fn from(x: &Owner) -> Self { Value::from(x.inner()) }
}
impl From<&EntityId> for Value {
  fn from(x: &EntityId) -> Self { Value::from(x.inner()) }
}
//</editor-fold>
