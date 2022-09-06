---
title: "BTCPay Server"
date: 2022-04-26T13:40:30+08:00
---

# BTCPay Server & Pirate Chain


## Quick Start

Requirements: Docker

Tested on Linux

1. Download the `docker-compose.yml` for this project
2. Edit BTCPAYSERVER_starting_height and BTCPAYSERVER_fvk. They
should set to your receiving wallet birth height and full viewing key.
3. Launch the server

```
$ wget https://raw.githubusercontent.com/hhanh01/pirate-btcpay/main/docker/docker-compose.yml
$ vim docker-compose.yml
$ docker-compose -f docker-compose.yml up default
```

## Configuration

- Navigate to [BTCPayserver](http://localhost:14142)
and create the administrator account
{{%img "2022-09-06_20-25-28.png" %}}
- Create your store
{{%img "2022-09-06_20-28-48.png" %}}
- Make sure you set the default currency to `ARRR`
(you must type it in)
- Do not set a wallet or a lightning node,
{{%img "2022-09-06_20-29-28.png" %}}
- Instead, go to Settings then Pirate,
{{%img "2022-09-06_20-30-55.png" %}}
- Click on "Modify"
{{%img "2022-09-06_20-31-52.png" %}}
- Enable the Wallet. Save and return to the previous page
{{%img "2022-09-06_20-32-07.png" %}}
- Now create a test app
{{%img "2022-09-06_20-33-01.png" %}}
- Set the app name and leave it as Point of Sale
{{%img "2022-09-06_20-34-10.png" %}}
- Save the new app and then click on View
- Choose one of the item for sale and Pay for it
{{%img "2022-09-06_20-45-54.png" %}}
- You should see a payment code
{{%img "2022-09-06_20-46-34.png" %}}
- Use your wallet to pay for it
- Once it is confirmed, the invoice should be paid

Your server is now setup.

## Next steps

Refer to the BTCPayserver documentation for the next steps.

- You may want to configure a reverse proxy to isolate your server
from the internet and add SSL
- If you use Wordpress or Shopify, there are integration
plugins for these platforms
