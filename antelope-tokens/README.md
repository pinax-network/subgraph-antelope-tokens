# Substreams: `Antelope Tokens`

### Example queries

- `code:eosio.token`
- `symcode:EOS`
- `sym:4,EOS`
- `owner:eosio.stake`
- `owner:eosio.stake && code:eosio.token`
- `token:4,EOS@eosio.token`

### Available query fields

These are the expressions that can be used in queries:

- `contract:<contract>` - token contract account name
- `token:<extended_symbol>` - extended token symbol

Queries can include `&&` and `||` logical operands, as well as `(` and `)` parenthesis.

## Graph

```mermaid
graph TD;
  graph_out[map: graph_out];
  graph_out:params[params] --> graph_out;
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> graph_out;
  events:map_events --> graph_out;
  events:map_events[map: events:map_events];
  events:block_index:map_db_ops --> events:map_events;
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> events:block_index:index_blocks;
  events:block_index:map_db_ops[map: events:block_index:map_db_ops];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> events:block_index:map_db_ops;
```

## Modules

```bash
Name: graph_out
Initial block: 0
Kind: map
Input: params:
Input: source: sf.substreams.v1.Clock
Input: map: events:map_events
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: dc28d1962740e7680fd648923f9ee2957700c613

Name: events:map_events
Initial block: 0
Kind: map
Input: map: events:block_index:map_db_ops
Output Type: proto:antelope.tokens.v1.Events
Hash: 0914a5441e13fefa1efb950947b8c506eab1a2e7

Name: events:block_index:index_blocks
Initial block: 0
Kind: index
Input: source: sf.antelope.type.v1.Block
Output Type: proto:sf.substreams.index.v1.Keys
Hash: 15f03d68c8b56336999fac6be2a3d1c6d558288b

Name: events:block_index:map_db_ops
Initial block: 0
Kind: map
Input: source: sf.antelope.type.v1.Block
Block Filter: (using *events:block_index:index_blocks*): `&{db.table:accounts || db.table:stat}`
Output Type: proto:sf.antelope.type.v1.DBOps
Hash: 9df5eeba09d51a87ea9eaf3ca18a13111cc495b1
```
