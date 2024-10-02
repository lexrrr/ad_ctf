create table `notify24`.ip_set
(
    id          bigint auto_increment
        primary key,
    created_at  datetime(6)  null,
    description varchar(255) null,
    name        varchar(255) null,
    owner       varchar(255) null,
    ips         mediumtext   null,
    constraint UK_bal03fkswx8a9vkeimf3lk54m
        unique (name)
);

CREATE INDEX idx_ip_set_id ON `notify24`.ip_set(id);
CREATE INDEX idx_ip_set_owner ON `notify24`.ip_set(owner);

create table `notify24`.notification_response
(
    id              bigint auto_increment primary key,
    created_at      datetime(6)  null,
    status          varchar(255) null,
    uuid            varchar(255) null,
    message          mediumtext   null,
    recipient        varchar(255) null,
    send_at          datetime(6)  null
);

CREATE INDEX idx_notification_response_id ON `notify24`.notification_response(id);

create table `notify24`.received_notification
(
    id          varchar(36)  not null primary key,
    created_at  datetime(6) null,
    message     mediumtext  null,
    received_at datetime(6) null
);

create table `notify24`.user
(
    id            bigint auto_increment primary key,
    created_at    datetime(6) null,
    email         varchar(50) null,
    password_hash varchar(64) null,
    constraint UK_ob8kqyqqgmefl0aco34akdtpe
        unique (email)
);

CREATE INDEX idx_user_email ON `notify24`.user(email);