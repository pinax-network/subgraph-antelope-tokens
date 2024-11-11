-------------------------------------------------
-- Meta tables to store Substreams information --
-------------------------------------------------

CREATE TABLE IF NOT EXISTS cursors
(
    id        String,
    cursor    String,
    block_num Int64,
    block_id  String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

-----------------------------------------------------------
-- Tables to store the raw events without any processing --
-----------------------------------------------------------

-- The table to store all transfers. This uses the trx_id as first primary key so we can use this table to do
-- transfer lookups based on a transaction id.
CREATE TABLE IF NOT EXISTS transfer_events
(
    trx_id       String,
    action_index UInt32,
    -- contract & scope --
    contract     String,
    symcode      String,
    -- data payload --
    from         String,
    to           String,
    quantity     String,
    memo         String,
    -- extras --
    precision    UInt32,
    amount       Int64,
    value        Float64,
    -- meta --
    block_num    UInt64,
    timestamp    DateTime
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (trx_id, action_index)
        ORDER BY (trx_id, action_index);

-- The table to store all account balance changes from the database operations. This uses the account and block_num as
-- first primary keys so we can use this table to lookup the account balance from a certain block number.
CREATE TABLE IF NOT EXISTS balance_change_events
(
    trx_id        String,
    action_index  UInt32,
    -- contract & scope --
    contract      String,
    symcode       String,
    -- data payload --
    account       String,
    balance       String,
    balance_delta Int64,
    -- extras --
    precision     UInt32,
    amount        Int64,
    value         Float64,
    -- meta --
    block_num     UInt64,
    timestamp     DateTime
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (account, block_num, trx_id, action_index)
        ORDER BY (account, block_num, trx_id, action_index);

-- The table to store all token supply changes from the database operations. This uses the account and block_num as
-- first primary keys so we can use this table to lookup token supplies from a certain block number.
CREATE TABLE IF NOT EXISTS supply_change_events
(
    trx_id       String,
    action_index UInt32,
    -- contract & scope --
    contract     String,
    symcode      String,
    -- data payload --
    issuer       String,
    max_supply   String,
    supply       String,
    supply_delta Int64,
    -- extras --
    precision    UInt32,
    amount       Int64,
    value        Float64,
    -- meta --
    block_num    UInt64,
    timestamp    DateTime
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (contract, block_num, trx_id, action_index)
        ORDER BY (contract, block_num, trx_id, action_index);

-- Table to contain all 'eosio.token:issue' transactions
CREATE TABLE IF NOT EXISTS issue_events
(
    trx_id       String,
    action_index UInt32,
    -- contract & scope --
    contract     String,
    symcode      String,
    -- data payload --
    issuer       String,
    to           String,
    quantity     String,
    memo         String,
    -- extras --
    precision    UInt32,
    amount       Int64,
    value        Float64,
    -- meta --
    block_num    UInt64,
    timestamp    DateTime
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (contract, symcode, to, amount, trx_id, action_index)
        ORDER BY (contract, symcode, to, amount, trx_id, action_index);

-- Table to contain all 'eosio.token:retire' transactions --
CREATE TABLE IF NOT EXISTS retire_events
(
    trx_id       String,
    action_index UInt32,
    -- contract & scope --
    contract     String,
    symcode      String,
    -- data payload --
    from         String,
    quantity     String,
    memo         String,
    -- extras --
    precision    UInt32,
    amount       Int64,
    value        Float64,
    -- meta --
    block_num    UInt64,
    timestamp    DateTime
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (contract, symcode, amount, trx_id, action_index)
        ORDER BY (contract, symcode, amount, trx_id, action_index);

-- Table to contain all 'eosio.token:create' transactions
CREATE TABLE IF NOT EXISTS create_events
(
    trx_id         String,
    action_index   UInt32,
    -- contract & scope --
    contract       String,
    symcode        String,
    -- data payload --
    issuer         String,
    maximum_supply String,
    -- extras --
    precision      UInt32,
    amount         Int64,
    value          Float64,
    -- meta --
    block_num      UInt64,
    timestamp      DateTime
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (contract, symcode, trx_id, action_index)
        ORDER BY (contract, symcode, trx_id, action_index);

-----------------------------------------------
-- Tables to store the extracted information --
-----------------------------------------------

-- Table to store up to date balances per account and token
CREATE TABLE IF NOT EXISTS account_balances
(
    account              String,

    contract             String,
    symcode              String,
    balance              String,

    precision            UInt32,
    amount               Int64,
    value                Float64,

    updated_at_block_num UInt64,
    updated_at_timestamp DateTime
)
    ENGINE = ReplacingMergeTree(updated_at_block_num)
        PRIMARY KEY (account, contract, symcode)
        ORDER BY (account, contract, symcode);

-- Table to store up to date token supplies
CREATE TABLE IF NOT EXISTS token_supplies
(
    contract             String,
    symcode              String,

    issuer               String,
    max_supply           String,
    supply               String,

    precision            UInt32,
    amount               Int64,
    value                Float64,

    updated_at_block_num UInt64,
    updated_at_timestamp DateTime
)
    ENGINE = ReplacingMergeTree(updated_at_block_num)
        PRIMARY KEY (contract, symcode, issuer)
        ORDER BY (contract, symcode, issuer);

-- Table to store token transfers primarily indexed by the 'from' field --
CREATE TABLE IF NOT EXISTS transfers_from
(
    trx_id       String,
    action_index UInt32,

    contract     String,
    symcode      String,

    from         String,
    to           String,
    quantity     String,
    memo         String,

    precision    UInt32,
    amount       Int64,
    value        Float64,

    block_num    UInt64,
    timestamp    DateTime
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (from, to, trx_id, action_index)
        ORDER BY (from, to, trx_id, action_index);

-- Table to store token transfers primarily indexed by the 'to' field
CREATE TABLE IF NOT EXISTS transfers_to
(
    trx_id       String,
    action_index UInt32,

    contract     String,
    symcode      String,

    from         String,
    to           String,
    quantity     String,
    memo         String,

    precision    UInt32,
    amount       Int64,
    value        Float64,

    block_num    UInt64,
    timestamp    DateTime
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (to, from, trx_id, action_index)
        ORDER BY (to, from, trx_id, action_index);

-- Table to store token transfers primarily indexed by the 'block_num' field
CREATE TABLE IF NOT EXISTS transfers_block_num
(
    trx_id       String,
    action_index UInt32,

    contract     String,
    symcode      String,

    from         String,
    to           String,
    quantity     String,
    memo         String,

    precision    UInt32,
    amount       Int64,
    value        Float64,

    block_num    UInt64,
    timestamp    DateTime
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_num, trx_id, action_index)
        ORDER BY (block_num, trx_id, action_index);

---------------------------------------------------------
-- Materialized views to populate the extracted tables --
---------------------------------------------------------

CREATE MATERIALIZED VIEW account_balances_mv
    TO account_balances
AS
SELECT account,
       contract,
       symcode,
       balance,
       precision,
       amount,
       value,
       block_num AS updated_at_block_num,
       timestamp AS updated_at_timestamp
FROM balance_change_events;

CREATE MATERIALIZED VIEW token_supplies_mv
    TO token_supplies
AS
SELECT contract,
       symcode,
       issuer,
       max_supply,
       supply,
       precision,
       amount,
       value,
       block_num AS updated_at_block_num,
       timestamp AS updated_at_timestamp
FROM supply_change_events;

CREATE MATERIALIZED VIEW transfers_from_mv
    TO transfers_from
AS
SELECT trx_id,
       action_index,
       contract,
       symcode,
       from,
       to,
       quantity,
       memo,
       precision,
       amount,
       value,
       block_num,
       timestamp
FROM transfer_events;

CREATE MATERIALIZED VIEW transfers_to_mv
    TO transfers_to
AS
SELECT trx_id,
       action_index,
       contract,
       symcode,
       from,
       to,
       quantity,
       memo,
       precision,
       amount,
       value,
       block_num,
       timestamp
FROM transfer_events;

CREATE MATERIALIZED VIEW transfers_block_num_mv
    TO transfers_block_num
AS
SELECT trx_id,
       action_index,
       contract,
       symcode,
       from,
       to,
       quantity,
       memo,
       precision,
       amount,
       value,
       block_num,
       timestamp
FROM transfer_events;