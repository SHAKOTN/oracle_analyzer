## Blazingly Fast Oracle Data downloader

This project is made for those who want to download data from Oracles
Powered with async Rust and Tokio, this project can download data from Oracle database in a very fast way.

## Usage
- Make sure you have Rust installed: https://www.rust-lang.org/tools/install
- Modify constants in `src/constants.rs` to your needs and add/remove oracles you need data from.
- Create `.env` file and fill in `ETHNODEURL`. I suggest using pokt network community nodes.
- Then run `cargo build` and `cargo run`

Files should appear in project root directory in a format of `<ORACLE_NAME>_oracle_data.csv`
