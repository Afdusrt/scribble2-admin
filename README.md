# scribble2-admin
Simple CLI tool for mass-submitting Color Book level runs to Speedrun.com.
<b>For submitting co-op runs, you (the api key supplied) has to be a moderator on the Color Book leaderboard (verifiers do not cut it, due to speedrun.com/api/v1/runs POST limitations.</b>

## Dependencies
<p>The program relies on:</p>

```rust
std::process::Command("curl").arg(...
```
to send requests to speedrun.com, so make sure curl is on your path.

## Using it

```shell
$ scribble2 src_api_key text_file_path
```

<p>To get your speedrun.com api key, go into your Profile Settings -> API Key -> Show API Key</p>
<p><b>TEXT FILE FORMATTING:</b></p>

```shell
player1ID,player2ID | Map Name | 1:23.456 | Gear/Gearless | main video link | comment(additional povs) | word 'split' if you are doing split duo, otherwise this column is not needed
```
<p>Each line is it's own run. Use UTF-8 encoding.</p>
<p>Example quad submission of 2 runs:</p>

```shell
8r27kvgx,xy57n7v8,8e6rpq7j,8wlw6w4j|Mountain    |8.97|Gearless|https://youtu.be/RNA7siYUTDE?t=1227|https://youtu.be/j-7-rl-Tvsw https://www.youtube.com/watch?v=Zp9ljnAXRYM https://youtu.be/-G3fb8jxzPg
8r27kvgx,xy57n7v8,8e6rpq7j,8wlw6w4j|Party Island|12.3|Gearless|https://youtu.be/RNA7siYUTDE?t=1255|https://youtu.be/j-7-rl-Tvsw https://www.youtube.com/watch?v=Zp9ljnAXRYM https://youtu.be/-G3fb8jxzPg
```
(entires get .trim()ed so spaces do not matter, using a spreadsheet processor like libreoffice calc, to create the text file as a csv, shouldnt cause any problems, as long as you check "edit filter settings" use "|" as the delimiter, no string delimiter and UTF-8 encoding)
