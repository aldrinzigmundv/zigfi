# zigfi

![Alt text](screenshots/1.png?raw=true "Screenshot 1")
![Alt text](screenshots/2.png?raw=true "Screenshot 2")

zigfi is an open-source stocks, commodities and cryptocurrencies price monitoring CLI app, written fully in Rust, where you can organize assets you're watching easily into watchlists.

This used to be just a personal app of mine until I decided to share it. The app only consisted of a few hours of coding and is meant to just work, instead of being a community developed app, so there are a lot of issues with the code regarding readability, repetitions, inconsistency, error handling and stuff.

I'll try to improve it as soon as possible. Also, I'm planning to add additional features, like news and maybe trading on the app itself in the future. Please, don't hesitate to file any issues if there are any bugs or suggest features.

I haven't tested the app yet with Windows but exe release is available if anyone wants to try.

Also, the app gets its data from the Yahoo Finance API.

## Quickstart
Here are the things you can do:
```
zigarg show <watchlist name>
zigarg create <watchlist name> <optional: ticker/s>
zigarg delete <watchlist name>
zigarg add <watchlistname> <ticker/s>
zigarg remove <watchlist> <ticker/s>
zigarg search <name of asset>
zigarg list
zigarg help
```

Releases are on Github at the right side of the repo.

You can visit my website at `aldrinzigmund.com`. Donations are also welcome there via Monero, if you want to support me really work on the app further.