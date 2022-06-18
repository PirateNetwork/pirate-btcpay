---
title: Server Status
weight: 20
---

Both of these functions return the current height

```js
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
```
