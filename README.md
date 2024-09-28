# Subgraph: `Antelope Tokens`

> Token Balances & Supply
>
> WAX, EOS, Ultra, Telos...
> [`sf.antelope.type.v1.Block`](https://buf.build/pinax/firehose-antelope/docs/main:sf.antelope.type.v1)

- [x] **Balances**
- [x] **Supply** & **Max Supply**

## Chains

- **API Key**: <https://thegraph.com/studio/apikeys/>
- **Base URL**: <https://gateway.thegraph.com/api>
- **Query URL format**: `{base_url}`/api/`{api-key}`/subgraphs/id/`{subgraph_id}`

| Chain | Subgraph ID |
| ----- | ----------- |
| WAX   | [`6Tt5mHVNDyAo3KbsYMUeemmzs2381vXUquXw75EnG9cW`](https://thegraph.com/explorer/subgraphs/6Tt5mHVNDyAo3KbsYMUeemmzs2381vXUquXw75EnG9cW?view=Query&chain=arbitrum-one) |
| EOS   | [`Ce1om4KPxZHwFxhtz2pVuCD4AUiKisrYecHVWsvEW6MU`](https://thegraph.com/explorer/subgraphs/Ce1om4KPxZHwFxhtz2pVuCD4AUiKisrYecHVWsvEW6MU?view=Query&chain=arbitrum-one) |
| Kylin   | [`B7YqehhCQyZmqCPL6raVv8ncqzLq69EFirjf7Gnfiv7A`](https://thegraph.com/explorer/subgraphs/B7YqehhCQyZmqCPL6raVv8ncqzLq69EFirjf7Gnfiv7A?view=Query&chain=arbitrum-one) |

## GraphQL

**Balances by Owner**

```graphql
query BalanceByOwner{
  balances(first: 20, orderBy:block__number, orderDirection:desc,
    where:{ owner:"swap.alcor" }) {
    token{
      code
      symcode
      precision
    }
    balance
  }
}
```

**Tokens by Top Holders**

```graphql
query TokensHolders {
  tokens(first:20, orderBy:block__number, orderDirection:desc){
    code
    symcode
    balances(first:5, orderBy: balance, orderDirection: desc) {
      owner
      balance
    }
  }
}
```

**Token Supply**

```graphql
query Supply {
  supplies(first:20, orderBy:block__number, orderDirection:desc){
    supply
    maxSupply
    token{
      code
      symcode
    }
  }
}
```

**Recent Balances Changes**

```graphql
query RecentBalancesChanges {
  balances(first: 20, orderBy:block__number, orderDirection:desc) {
    owner
    balance
    block{
      timestamp
      number
    }
    token{
      code
      symcode
      precision
    }
  }
}
```

## Substreams Modules

```mermaid
graph TD;
  graph_out[map: graph_out];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> graph_out;
```

## Subgraph deployment

```bash
graph indexer rules prepare --network arbitrum-one <Qm>
graph indexer allocations create <Qm> arbitrum-one 100
```
