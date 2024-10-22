# 🎽 What to wear running

[![MIT license](https://img.shields.io/github/license/kesyog/wtw-running?style=flat-square)](https://github.com/kesyog/wtw-running/blob/master/LICENSE)
[![build status](https://img.shields.io/github/workflow/status/kesyog/wtw-running/Rust?style=flat-square)](https://github.com/kesyog/wtw-running/actions)

A CLI tool and Alexa skill that recommends a running outfit for you based on weather conditions
pulled from [OpenWeatherMap](https://openweathermap.org) and given user preferences.

Gear data ripped off from a flaky [Runner's World page](https://www.runnersworld.com/training/a20803133/what-to-wear/)
that tends to spontaneously disappear for periods of time.

There are quite a few similar and likely better tools out there. This is mostly just an excuse to
re-familiarize myself with basic Rust syntax and concepts.

## Usage

### CLI tool

Run the following from the repo root:

```bash
cargo build --release -p wtw-running-cli
OWM_API_KEY=<OpenWeatherMap API key goes here> target/release/wtwr
```

You can optionally supply the OpenWeatherMap API key via a .env file by enabling the `dotenv_key`
feature.

### Alexa skill

This method requires targeting `x86_64-unknown-linux-musl`. See <https://github.com/awslabs/aws-lambda-rust-runtime/issues/17>
for alternate methods (e.g. building inside a Docker container).

First install the musl toolchain:

```bash
sudo apt install musl musl-tools
rustup target add x86_64-unknown-linux-musl
```

Then run the following from the repo root to build the skill:

```bash
cd skill && ./build.sh
```

This generates a rust.zip file that can be used as a custom runtime for an AWS Lambda function.
