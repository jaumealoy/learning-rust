# Introduction
We want to build a REST API to query cryptocurrency prices. Seems simple but we will introduce some features that will increase the difficulty of this app:

- We will fetching real time prices using websockets.
- We will be able to get a price between currencies that do not have a market

The end goal is to have an endpoint like `/v1/price/BASE/QUOTE` where `BASE` is a symbol name and `QUOTE` is another symbol name.

In previous projects we have used asynchronous programming (with Tokio) but we do not have created an API nor used a WebSocket client. Therefore, we must learn how to create a webserver and use a websocket client.

Let's get started.