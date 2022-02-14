<center>
    <h1>Sekkei</h1>
</center>

This is a parser for the osu! [.osu file specifications](https://osu.ppy.sh/wiki/en/Client/File_formats/Osu_%28file_format%29), and it intends to create an accurate and detailed structure on the likes of what osu! would standardly parse it as. Primarily, this was made to parse a .osu file into a human readable structure that can be used for [akairo](), osu!Katagiri's customly-made difficulty calculator.

## Requirements and Prerequisites
* Rust (Nightly)

## Technical Standpoint
To make things more convenient, I very easily could have just used JavaScript to parse beatmaps, and create the difficulty calculator with that. I felt JavaScript's dynamic and deterministic type system would only serve to inhibit the performance and quality of the project, and thus I went with a language to parse the [.osu format](https://osu.ppy.sh/wiki/en/Client/File_formats/Osu_%28file_format%29) in a quick and performant language that would suit our purposes of supporting any heavy-weight load.

### Benchmarks

Unfortunately, the only benchmark results we hold right now is ones for ours. When we coordinate a bunch of libraries together and reasonably bench them with a base point being osu!lazer, we will only rationally be able to provide out own results, with the use of Criterion.

|                             | `osu!` | `sekkei`|
|-----------------------------|--------|---------|
| Beatmap parsing performance | 1x     | ?       |
| Base size                   | ❌    | 3.282mb |
| Full hit object handling    | ✅    | ✅      |
| 


#### Current Results:
```
sekkai-parser/test unpack
                        time:   [1.4284 ms 1.4436 ms 1.4566 ms]
                        thrpt:  [638.27 KiB/s 644.00 KiB/s 650.87 KiB/s]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
```