# Mulingo
A Multi-Language configuration component.

## DDL
```mysql

CREATE TABLE t_muligo (
    id VARCHAR(32) PRIMARY KEY,
    name_space VARCHAR(60) NOT NULL,
    msg_key VARCHAR(200) NOT NULL,
    lang_key VARCHAR(20) NOT NULL,
    msg_content VARCHAR(2048) NOT NULL,
    create_at TIMESTAMP NOT NULL,
    update_at TIMESTAMP NOT NULL
) comment 'multi language message configuration';

```
