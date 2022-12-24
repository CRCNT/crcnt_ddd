# Mulingo
A Multi-Language configuration component.

## DDL
```mysql
CREATE TABLE t_mulingo (
  id VARCHAR(32) PRIMARY KEY,
  owner VARCHAR(60) NOT NULL,
  name_space VARCHAR(60) NOT NULL,
  msg_key VARCHAR(200) NOT NULL,
  lang_key VARCHAR(20) NOT NULL,
  version VARCHAR(20) NOT NULL,
  msg_content VARCHAR(2048) NOT NULL,
  memo VARCHAR(200) NULL,
  create_at TIMESTAMP NOT NULL,
  update_at TIMESTAMP NOT NULL
) comment 'multi language message configuration'

```

## Usage

### Dependencies
- `mysql_async` to provide `Conn` to run the store functions
- `tracing` & `tracing_subscriber` to enable tracing and logging

### Domain Application
The domain Application is the function entrance which need be initialized at first.
```rust
use crcnt_mulingo::includes::Application;
fn create_application() {
  let pool: Pool = Pool::new("mysql://promo_user:promo_userpw@localhost:3306/promo");
  let application = Application::new(pool);
  application
}

```

#### Create Mulingo
The Application's `create_mulingo` need seven arguments:
- owner: the owner of the item
- ns: namespace of the item
- lang_key: the language key, e.g.: `en`, `zh_CN`, ...
- msg_key: the item key
- msg: the item content
- memo: the item memo, optional

```rust
use crcnt_mulingo::includes::*;

async fn create_mulingo() {
  let ns: MulingoNameSpace = "com.payby.promotion".into();
  let owner: MulingoOwner = "SYS_PROMOTION".into();
  let lang_key: MulingoLangKey = "en".into();
  let version: MulingoVersion = "0.1.2".into();
  let msg_key: MulingoMsgKey = "EC_600001".into();
  let msg: MulingoMsgContent = "The promotion code is overflow".into();
  let memo: Option<MulingoMemo> = Some("erro code for overflow".into());

  let entity = app.create_mulingo(owner,
                                  ns,
                                  lang_key,
                                  msg_key,
                                  version,
                                  msg,
                                  memo)
    .await;
}
```

#### Fetch Mulingo
The Application's `fetch_latest_mulingo` can get the mulingo item of the latest version.

```rust
use crcnt_mulingo::includes::*;

async fn create_mulingo() {
  let ns: MulingoNameSpace = "com.payby.promotion".into();
  let owner: MulingoOwner = "SYS_PROMOTION".into();
  let lang_key: MulingoLangKey = "en".into();
  let msg_key: MulingoMsgKey = "EC_600001".into();

  let mulingo = app.fetch_latest_mulingo(&owner, &ns, &msg_key, &lang_key).await;
}
```
