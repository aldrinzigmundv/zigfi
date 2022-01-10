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
zigfi (shows "default" watchlist)
zigfi new <watchlist name> <optional: ticker/s>
zigfi show <watchlist name> <optional: interval> (interval can be "1d", "1mo" or "1y")
zigfi delete <watchlist name>
zigfi add <watchlist name> <ticker/s>
zigfi remove <watchlist name> <ticker/s>
zigfi search <name of asset>
zigfi list (lists saved watchlist/s)
zigfi colorswap (swaps Green and Red for some East Asian users)
zigfi help
```

`zigfi show` also supports piping. Default output is string. Add `--json` flag for JSON.

Releases are on Github at the right side of the repo.

You can visit my website at `aldrinzigmund.com`. Donations are also welcome via Monero, if you want to support me work on the app further:

86cQoPfKTJ2bRfGH5Ts2kzaXCRcVRiX8CUHKc9xmeUmQ8YM8Uzk9S97T5gQaqYu58C9wuFK7opDH7cM9EJyR4V5LAq9RGv4