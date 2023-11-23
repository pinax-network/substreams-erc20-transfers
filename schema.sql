CREATE TABLE IF NOT EXISTS Transfers  (
    address FixedString(40),
    `from` String,
    `to` String,
    value String,
    transaction String
)
ENGINE = ReplacingMergeTree()
ORDER BY (address, `from`, `to`);