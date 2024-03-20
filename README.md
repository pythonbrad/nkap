# nkap
`nkap` is an utility to perform currency convertions in real time.

## Preview
![Preview](https://github.com/pythonbrad/nkap/assets/45305909/b8975061-a16e-4d70-8e05-61b65d1bfb23)

## About
`nkap` is a CLI curency converter based on [openexchangerates.org](https://openexchangerates.org).

## How to get nkap?

### By Installation

```
cargo install --git https://github.com/pythonbrad/nkap
```

### By Building
`nkap` is writting in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/) in order to compile it.

To build nkap:
```
git clone https://github.com/pythonbrad/nkap
cd nkap
cargo build --release
```

### Using Docker

```
docker build -t nkap-app .
```

## How config nkap?
`nkap` use the [openexchangerates.org](openexchangerates.org) API, and to work, we should get an API access from them. You can do it as follow:

1. Open the [openexchangerates.org](https://openexchangerates.org/signup/free) website and create an account for a free plan or whatever you want.
2. Login and open your dashboard.
3. On the sidebar, click on App IDs.
4. Copy your App ID.
5. In your environement variable, set your App ID as follow `APP_ID=<your_app_id>`.
Eg. `APP_ID=1a13d64adae7488eb9ca39158e28dc8f`

## Getting Started!
```
nkap --help
```  

## License
[MIT](LICENSE)
