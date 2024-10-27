# Block Index: `Antelope Tokens`

```bash
Name: index_blocks
Initial block: 0
Kind: index
Input: source: sf.antelope.type.v1.Block
Output Type: proto:sf.substreams.index.v1.Keys
Hash: e5eeb654cedab6f2c5c4b28437b64c9b92654cc9

Name: map_db_ops
Initial block: 0
Kind: map
Input: source: sf.antelope.type.v1.Block
Block Filter: (using *index_blocks*): `&{db.table:accounts || db.table:stat}`
Output Type: proto:sf.antelope.type.v1.DBOps
Hash: 54e0b8ffa79689af84b16d2a2cf800df55495fc7
```
