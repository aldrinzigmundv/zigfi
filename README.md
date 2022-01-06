# zigfi

![Alt text](screenshots/1.png?raw=true "Screenshot 1")
![Alt text](screenshots/2.png?raw=true "Screenshot 2")

zigfi is an open-source stocks, commodities and cryptocurrencies price monitoring CLI app, written fully in Rust, where you can organize assets you're watching into watchlists for easy access on the terminal.

This used to be just a personal app of mine until I decided to share it. I'm looking forward to adding more features in the future.

Also, I haven't tested the app yet with Windows but exe release is available if anyone wants to try. This is made mainly for GNU/Linux operating systems.

The app gets its data from the Yahoo Finance API.

## Quickstart
Here are the things you can do:
```
zigarg (shows "default" watchlist)
zigarg new <watchlist name> <optional: ticker/s>
zigarg show <watchlist name> <optional: interval> (interval can be "1d", "1mo" or "1y")
zigarg delete <watchlist name>
zigarg add <watchlist name> <ticker/s>
zigarg remove <watchlist name> <ticker/s>
zigarg search <name of asset>
zigarg list (lists saved watchlist/s)
zigarg colorswap (swaps Green and Red for some East Asian users)
zigarg help
```

Releases are on Github at the right side of the repo.

You can visit my website at `aldrinzigmund.com`. Donations are also welcome there via Monero, if you want to support me really work on the app further.