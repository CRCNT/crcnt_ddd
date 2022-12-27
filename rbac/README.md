# RBAC Module

## DDL
### Operator
```mysql
CREATE TABLE t_rbac_operator (
    id VARCHAR(32) PRIMARY KEY,
    profile_id VARCHAR(32) NULL,
    name VARCHAR(60) NOT NULL ,
    name_type VARCHAR(20) NOT NULL COMMENT 'NameType: LoginName, Email, Mobile',
    password VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL COMMENT 'Status: Active|Inactive',
    last_login_at TIMESTAMP,
    failed_times INT(4) NOT NULL,
    owner VARCHAR(50) NOT NULL,
    creator VARCHAR(32) NOT NULL,
    updater VARCHAR(32) NOT NULL,
    create_at TIMESTAMP NOT NULL,
    update_at TIMESTAMP NOT NULL,
    deleted BOOL
) COMMENT 'operators for rbac module';
-- the default password: passw0rd!
INSERT INTO `t_rbac_operator` VALUES ('0','0','ROOT','LoginName','6d8e53b0ba388cb82e806b6c7539f98aef51babc52cc23e3d85df35ea6251444','Active','2022-12-27 12:04:46',0,'SYS','SYS','SYS','2022-12-27 12:05:13','2022-12-27 12:05:16',0);
```

### Role
```mysql
CREATE TABLE t_rbac_role (
    id VARCHAR(32) PRIMARY KEY,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL ,
    description VARCHAR(200),
    level INT(4) NOT NULL,
    status VARCHAR(20) NOT NULL COMMENT 'Status: Active|Inactive',
    owner VARCHAR(50) NOT NULL,
    creator VARCHAR(32) NOT NULL,
    updater VARCHAR(32) NOT NULL,
    create_at TIMESTAMP NOT NULL,
    update_at TIMESTAMP NOT NULL,
    deleted BOOL
) COMMENT 'roles for rbac module';
```

### Feature
```mysql
CREATE TABLE t_rbac_feature (
    id VARCHAR(32) PRIMARY KEY,
    parent_id VARCHAR(32),
    code VARCHAR(50) NOT NULL,
    name VARCHAR(100) NOT NULL ,
    endpoint VARCHAR(255),
    description VARCHAR(200),
    status VARCHAR(20) NOT NULL COMMENT 'Status: Active|Inactive',
    creator VARCHAR(32) NOT NULL,
    updater VARCHAR(32) NOT NULL,
    create_at TIMESTAMP NOT NULL,
    update_at TIMESTAMP NOT NULL,
    deleted BOOL
) COMMENT 'features for rbac module';
```

### Operator's Roles
```mysql
CREATE TABLE t_rbac_operator_role (
    id VARCHAR(32) PRIMARY KEY,
    operator_id VARCHAR(32) NOT NULL,
    role_id VARCHAR(32) NOT NULL,
    owner VARCHAR(50) NOT NULL,
    create_at TIMESTAMP NOT NULL,
) COMMENT 'the relation of operator and role for rbac module';
```

### Role's Feature
```mysql
CREATE TABLE t_rbac_role_feature (
    id VARCHAR(32) PRIMARY KEY,
    role_id VARCHAR(32) NOT NULL,
    feature_id VARCHAR(32) NOT NULL,
    owner VARCHAR(50) NOT NULL,
    create_at TIMESTAMP NOT NULL
) COMMENT 'the relation of role and feature for rbac module';
```

### Session
```mysql
CREATE TABLE t_rbac_session (
    id VARCHAR(32) PRIMARY KEY,
    operator_id VARCHAR(32) NOT NULL,
    data VARCHAR(2048),
    login_at TIMESTAMP NOT NULL,
    last_hit_at TIMESTAMP NOT NULL,
    expire_at TIMESTAMP NOT NULL,
    owner VARCHAR(50) NOT NULL
) COMMENT 'session for rbac module';
```

### Access
```mysql
CREATE TABLE t_rbac_access (
    id VARCHAR(32) PRIMARY KEY,
    operator_id VARCHAR(32) NOT NULL,
    session_id VARCHAR(32) NOT NULL,
    feature_code VARCHAR(50) NOT NULL,
    control VARCHAR(20) NOT NULL COMMENT 'Control: Allowed,Forbidden',
    access_at TIMESTAMP NOT NULL,
    owner VARCHAR(50) NOT NULL
) COMMENT 'session for rbac module';
```
