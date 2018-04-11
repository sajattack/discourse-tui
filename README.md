# discourse-tui
Terminal UI for Discourse forums

![screenshot](https://i.imgur.com/Q4toKDd.png)

## Dependencies
* Cargo
* ncurses

## Setup
```sh
mkdir -p ~/.config/discourse-tui
cp theme.toml ~/.config/discourse-tui
cargo build --release
cp target/release/discourse-tui ~/.local/bin
```

## Run (unauthenticated)
```sh
discourse-tui <url>
```

## Setup Api Key

### First you need to register discourse-tui as the handler for discourse:// URIs
```sh
cp discourse-tui.desktop ~/.local/share/applications
```
Add this line to ~/.config/mimeapps.list, under \[Added Associations\] and \[Default Applications\]
```
x-scheme-handler/discourse=discourse-tui.desktop
```
You may need to logout for this to have an effect.

### Now, back to discourse-tui
```sh
discourse-tui --new-key <url>
```
Click the link
Click "Authorize"
You should see a terminal flash open briefly, this is discourse-tui opening and completing the authentication.

Now just run
```sh 
discourse-tui
```
The TUI will open with the URL you specified in the --new-key phase and you will be logged in.

## Troubleshooting
If discourse-tui doesn't flash open when you click "Authorize", you will need to manually copy the discourse://auth_redirect?payload=<base64> URI as an argument to discourse-tui.
Try opening the network tab of your browser's developer tools, refreshing the authorize page, clicking authorize, and look for this URI.
