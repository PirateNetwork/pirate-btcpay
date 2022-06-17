const chakram = require('chakram'), expect = chakram.expect;

describe('pirate-btcpay', async function () {
    it("should allow creating new accounts", async function () {
        const rep = await chakram.post('http://localhost:8000/create_account', {})
        expect(rep).to.have.status(200)
        expect(rep.body).to.deep.equal({
            account_index: 0,
            address: "zs1xflcmxuagwwn969htsej4mn3nuk7e3eeu3en4n4hkxajnqgmx5jsw0sa4p0r2ymqewf2ufefkfm"
        })
    })

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

    it("should give you the latest block height", async function () {
        const rep = await chakram.post('http://localhost:8000/get_height', {})
        expect(rep).to.have.status(200)
        expect(rep.body > 1000000)
    })

    it("should give you server status", async function () {
        const rep = await chakram.post('http://localhost:8000/sync_info', {})
        expect(rep).to.have.status(200)
        expect(rep.body.height > 1000000)
    })

    it("Scan blockchain", async function () {
        this.timeout(120000);
        const rep = await chakram.post('http://localhost:8000/request_scan', {})
        expect(rep).to.have.status(200)
    })

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

    it("list transactions", async function () {
        const rep = await chakram.post('http://localhost:8000/get_transfers', {
            in: true,
            account_index: 0,
            subaddr_indices: [0]
        })
        expect(rep).to.have.status(200)
        expect(rep.body).to.deep.equal({
            "in": [
                {
                    "address": "zs1xflcmxuagwwn969htsej4mn3nuk7e3eeu3en4n4hkxajnqgmx5jsw0sa4p0r2ymqewf2ufefkfm",
                    "amount": 99500000,
                    "confirmations": 31984,
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

    it("Get fee estimate", async function () {
        const rep = await chakram.post('http://localhost:8000/get_fee_estimate', {})
        expect(rep).to.have.status(200)
        expect(rep.body).to.deep.equal({
            "fee": 1000
        })
    })

    it("Get transfer by id", async function () {
        const rep = await chakram.post('http://localhost:8000/get_transfer_by_txid', {
            txid: "80ce417f3984ba936f344c4b2c9737740604767f5ac34a2c130925b015637835",
            account_index: 0
        })
        expect(rep).to.have.status(200)
        expect(rep.body).to.deep.equal({
            "transfer": {
                "address": "zs1xflcmxuagwwn969htsej4mn3nuk7e3eeu3en4n4hkxajnqgmx5jsw0sa4p0r2ymqewf2ufefkfm",
                "amount": 99500000,
                "confirmations": 31984,
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
})
