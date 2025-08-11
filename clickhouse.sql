create database event;

use event;

create table session
(
    client_id  Int32,
    uuid       UUID,
    device_id  UUID,
    created_at DateTime64(6)
)
    engine = MergeTree ORDER BY (created_at, client_id)
        SETTINGS index_granularity = 8192;

create table event
(
    event_id           UUID,
    client_id          Int32,
    uuid               UUID,
    event_type         UInt8,
    order_id           Nullable(String),
    tracking_id        Nullable(String),
    product_id         Nullable(String),
    product_option_id1 Nullable(String),
    product_option_id2 Nullable(String),
    product_quantity   Nullable(UInt8),
    price              Nullable(UInt32),
    amount             Nullable(UInt32),
    keyword            Nullable(String),
    sort_by            Nullable(String),
    page               Nullable(UInt8),
    board_id           Nullable(String),
    article_id         Nullable(String),
    login_id           Nullable(String),
    phone_number       Nullable(String),
    page_url           Nullable(String),
    category_id        Nullable(String),
    scroll_bucket      Nullable(UInt8),
    created_at         DateTime64(6)
)
    engine = MergeTree PARTITION BY toYYYYMMDD(created_at)
        ORDER BY (client_id, event_type, created_at, uuid)
        SETTINGS index_granularity = 8192;