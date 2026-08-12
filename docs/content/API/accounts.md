---
title: Accounts
weight: 10
---

{{%notice note%}}
Addresses returned by `create_account`/`create_address` may be Sapling
(`zs1...`) or Ironwood (`pirate1...`), depending on which pool the account
is pinned to (or the wallet's current default, for an account never
pinned). Both are opaque strings from the caller's perspective — no
special handling is needed either way.
{{%/notice %}}

## Create a new account

Create a new account and return its index and first address. An optional
`pool` field (`"sapling"` or `"ironwood"`) pins which shielded pool this
account — and every address later allocated to it via `create_address` —
generates addresses from, permanently; once pinned, an account's pool
never changes.

Omitting `pool` leaves the account unpinned: every future address for it
(including this first one) resolves the wallet's current default pool
fresh each time, so it automatically starts generating Ironwood addresses
on its own once Ironwood activates for the wallet, with no action needed
from the caller.

```js
it("should allow creating new accounts", async function () {
    const rep = await chakram.post('http://localhost:8000/create_account', {
        pool: "ironwood"
    })
    expect(rep).to.have.status(200)
    expect(rep.body).to.deep.equal({
        account_index: 0,
        address: "pirate1xflcmxuagwwn969htsej4mn3nuk7e3eeu3en4n4hkxajnqgmx5jsw0sa4p0r2ymqewf2ufefkfm"
    })
})
```


## Create a new sub address

Create a new diversified address for the given account, from whichever
pool that account is currently pinned to (or the wallet's current default,
if it was never pinned)

```js
it("should allow creating new address", async function () {
    const rep = await chakram.post('http://localhost:8000/create_address', {
        account_index: 0
    })
    expect(rep).to.have.status(200)
    expect(rep.body).to.deep.equal({
        address: "pirate1afgu28nemdza4exykhvp24d6jxja85jg44gu8k7xwusr5p09yery922nfn228hvagc33x5wkd4h",
        address_index: 1
    })
})
```

## List all accounts and balances

{{%notice warning%}}
`walletd` does not consider *outgoing* transactions. It is designed to 
handle incoming **PAYMENTS**. Therefore if you spend from these accounts,
the balance will be over-estimated.
{{%/notice %}}

```js
it("list accounts", async function () {
    const rep = await chakram.post('http://localhost:8000/get_accounts', {})
    expect(rep).to.have.status(200)
    expect(rep.body).to.deep.equal({
        "subaddress_accounts": [
            {
                "account_index": 0,
                "balance": 99500000,
                "base_address": "pirate1xflcmxuagwwn969htsej4mn3nuk7e3eeu3en4n4hkxajnqgmx5jsw0sa4p0r2ymqewf2ufefkfm",
                "label": "",
                "tag": "",
                "unlocked_balance": 99500000,
                "pool": "ironwood",
                "pool_pinned": true
            }
        ],
        "total_balance": 99500000,
        "total_unlocked_balance": 99500000
    })
})
```