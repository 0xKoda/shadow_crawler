# Dark Web Search

This is a simple Rust program that searches the dark web using the Ahmia search engine. It takes a search query as a command line argument and returns any matching links found on Ahmia.

This was made as a hobby to search for strings on the darkweb for osint purposes. 

## Installation

1. Install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
2. Clone this repository: `git clone https://github.com/your-username/dark-web-search.git`
3. Navigate to the project directory: `cd dark-web-search`
4. Build the project: `cargo build`

## Usage

Run the program with the following command:

```
cargo run <search_query>
```

Replace `<search_query>` with your desired search query.

## Dependencies

This program uses the following Rust crates:

- `reqwest` for making HTTP requests
- `select` for parsing HTML

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.