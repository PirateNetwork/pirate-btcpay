---
title: Payment
weight: 40
---

## Fee Estimate

For shielded payments, the fee is 0.01 m

```js
it("Get fee estimate", async function () {
    const rep = await chakram.post('http://localhost:8000/get_fee_estimate', {})
    expect(rep).to.have.status(200)
    expect(rep.body).to.deep.equal({
        "fee": 1000
    })
})
```

## Get transaction by ID

Given a tx ID and account index, return the transaction details

```js
it("Get transfer by id", async function () {
    const rep = await chakram.post('http://localhost:8000/get_transfer_by_txid', {
        txid: "80ce417f3984ba936f344c4b2c9737740604767f5ac34a2c130925b015637835",
        account_index: 0
    })
    expect(rep).to.have.status(200)
    delete(rep.body.transfer.confirmations)
    expect(rep.body).to.deep.equal({
        "transfer": {
            "address": "zs1xflcmxuagwwn969htsej4mn3nuk7e3eeu3en4n4hkxajnqgmx5jsw0sa4p0r2ymqewf2ufefkfm",
            "amount": 99500000,
            "height": 1915516,
            "fee": 0,
            "note": "",
            "payment_id": "",
            "subaddr_index": {
                "major": 0,
                "minor": 0
            },
            "suggested_confirmations_threshold": 6,
            "timestamp": 0,
            "txid": "80ce417f3984ba936f344c4b2c9737740604767f5ac34a2c130925b015637835",
            "type": "in",
            "unlock_time": 0
        },
        "transfers": []
    })
})
```

## Get all transactions

For a given account and a set of sub addresses, return all the transactions

```js
it("list transactions", async function () {
    const rep = await chakram.post('http://localhost:8000/get_transfers', {
        in: true,
        account_index: 0,
        subaddr_indices: [0]
    })
    expect(rep).to.have.status(200)
    delete(rep.body.in[0].confirmations)
    expect(rep.body).to.deep.equal({
        "in": [
            {
                "address": "zs1xflcmxuagwwn969htsej4mn3nuk7e3eeu3en4n4hkxajnqgmx5jsw0sa4p0r2ymqewf2ufefkfm",
                "amount": 99500000,
                "height": 1915516,
                "fee": 0,
                "note": "",
                "payment_id": "",
                "subaddr_index": {
                    "major": 0,
                    "minor": 0
                },
                "suggested_confirmations_threshold": 6,
                "timestamp": 0,
                "txid": "80ce417f3984ba936f344c4b2c9737740604767f5ac34a2c130925b015637835",
                "type": "in",
                "unlock_time": 0
            }
        ]
    })
})
```
