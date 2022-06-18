---
title: Scan
weight: 30
---

## Scan the blockchain

Scan the blockchain from the given `starting_height` or from the end of the previous scan.

```js
it("Scan blockchain", async function () {
    this.timeout(120000);
    const rep = await chakram.post('http://localhost:8000/request_scan', {})
    expect(rep).to.have.status(200)
})
```