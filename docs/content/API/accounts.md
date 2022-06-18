---
title: Accounts
weight: 10
---

## Create a new account

Create a new account and return its index and first address

```js
it("should allow creating new accounts", async function () {
    const rep = await chakram.post('http://localhost:8000/create_account', {})
    expect(rep).to.have.status(200)
    expect(rep.body).to.deep.equal({
        account_index: 0,
        address: "zs1xflcmxuagwwn969htsej4mn3nuk7e3eeu3en4n4hkxajnqgmx5jsw0sa4p0r2ymqewf2ufefkfm"
    })
})
```


## Create a new sub address

Create a new diversified address for the given account

```js
it("should allow creating new address", async function () {
    const rep = await chakram.post('http://localhost:8000/create_address', {
        account_index: 0
    })
    expect(rep).to.have.status(200)
    expect(rep.body).to.deep.equal({
        address: "zs1afgu28nemdza4exykhvp24d6jxja85jg44gu8k7xwusr5p09yery922nfn228hvagc33x5wkd4h",
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
                "base_address": "zs1xflcmxuagwwn969htsej4mn3nuk7e3eeu3en4n4hkxajnqgmx5jsw0sa4p0r2ymqewf2ufefkfm",
                "label": "",
                "tag": "",
                "unlocked_balance": 99500000
            }
        ],
        "total_balance": 99500000,
        "total_unlocked_balance": 99500000
    })
})
```