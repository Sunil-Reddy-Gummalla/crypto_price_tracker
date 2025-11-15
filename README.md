# Crypto Price Tracker

A simple cryptocurrency price tracker built with Rust. This application provides a web server that can be used to fetch the current price of a given cryptocurrency in USD.

## Features

-   Fetches cryptocurrency prices from the [CoinGecko API](https://www.coingecko.com/en/api).
-   Provides a simple JSON API to get the price of a specific coin.
-   Built with modern Rust libraries:
    -   [Axum](https://github.com/tokio-rs/axum) for the web server.
    -   [Tokio](https://tokio.rs/) as the asynchronous runtime.
    -   [Reqwest](https://github.com/seanmonstar/reqwest) for making HTTP requests.
    -   [Serde](https://serde.rs/) for JSON serialization and deserialization.

## Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) (2024 edition or later)

## Getting Started

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/crypto_price_tracker.git
    cd crypto_price_tracker
    ```

2.  **Build the project:**

    ```bash
    cargo build
    ```

3.  **Run the application:**

    ```bash
    cargo run
    ```

    The server will start on `0.0.0.0:3000`.

## Usage

To fetch the price of a cryptocurrency, make a GET request to the `/fetch-prices/{coin}` endpoint, where `{coin}` is the ID of the coin on CoinGecko (e.g., `bitcoin`, `ethereum`).

### Example

```bash
curl http://localhost:3000/fetch-prices/bitcoin
```

The server will respond with a JSON object containing the coin's ID and its current price in USD:

```json
{
  "coin": "bitcoin",
  "price": 68000.50
}
```