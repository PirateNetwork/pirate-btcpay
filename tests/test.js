const chakram = require('chakram'), expect = chakram.expect;

describe('pirate-btcpay', async function () {
    var id_account;

    it("should allow creating new accounts", async function () {
        const rep = await chakram.post('http://localhost:8000/create_account', {})
        expect(rep).to.have.status(200)
    })

    it("should allow creating new address", async function () {
        const rep = await chakram.post('http://localhost:8000/create_address', {
            account_index: 0
        })
        expect(rep).to.have.status(200)
    })

    it("should give you the latest block height", async function () {
        const rep = await chakram.post('http://localhost:8000/get_height', {})
        expect(rep).to.have.status(200)
        expect(rep.body > 1000000)
    })

    it("should give you server status", async function () {
        const rep = await chakram.post('http://localhost:8000/sync_info', {})
        expect(rep).to.have.status(200)
        expect(rep.body > 1000000)
    })

    it("list accounts", async function () {
        const rep = await chakram.post('http://localhost:8000/get_accounts', {})
        expect(rep).to.have.status(200)
    })

    it("list transactions", async function () {
        const rep = await chakram.post('http://localhost:8000/get_transfers', {
            in: true,
            account_index: 0,
            subaddr_indices: [0]
        })
        expect(rep).to.have.status(200)
    })

    it("Scan blockchain", async function () {
        const rep = await chakram.post('http://localhost:8000/request_scan', {})
        expect(rep).to.have.status(200)
    })

    it("Get fee estimate", async function () {
        const rep = await chakram.post('http://localhost:8000/get_fee_estimate', {})
        expect(rep).to.have.status(200)
    })

    it("Get transfer by id", async function () {
        const rep = await chakram.post('http://localhost:8000/get_transfer_by_txid', {
            txid: "80ce417f3984ba936f344c4b2c9737740604767f5ac34a2c130925b015637835",
            account_index: 0
        })
        expect(rep).to.have.status(200)
    })
})
