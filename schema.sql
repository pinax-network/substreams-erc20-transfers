CREATE TABLE IF NOT EXISTS transfers  (
    "id" String,
    contract FixedString(40),
    `from` String,
    `to` String,
    value String,
    tx_id String,
    block_num   UInt32(),
    timestamp       DateTime64(3, 'UTC'),
)
ENGINE = MergeTree PRIMARY KEY ("id")
ORDER BY (id,block_num,timestamp);

-- Indexes for block_number
ALTER TABLE transfers ADD INDEX transfers_block_number_index block_num TYPE minmax;

-- MV for contract --
CREATE MATERIALIZED VIEW transfers_contract_historical_mv
ENGINE = MergeTree()
ORDER BY (contract, `from`,`to`)
POPULATE
AS SELECT * FROM transfers;

-- MV for from --
CREATE MATERIALIZED VIEW transfers_from_historical_mv
ENGINE = MergeTree()
ORDER BY (`from`, contract)
POPULATE
AS SELECT * FROM transfers;

-- MV for from --
CREATE MATERIALIZED VIEW transfers_to_historical_mv
ENGINE = MergeTree()
ORDER BY (`to`, contract)
POPULATE
AS SELECT * FROM transfers;

-- MV for tx_id --
CREATE MATERIALIZED VIEW transfers_tx_id_historical_mv
ENGINE = MergeTree()
ORDER BY (`tx_id`, contract)
POPULATE
AS SELECT * FROM transfers;