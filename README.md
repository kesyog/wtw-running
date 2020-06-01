# 🎽 What to wear running
Recommends a running outfit for you based on weather conditions pulled from [OpenWeatherMap](https://openweathermap.org)
and user preferences.

Gear data ripped off from a flaky [Runner's World page](https://www.runnersworld.com/training/a20803133/what-to-wear/)
that tends to spontaneously disappear for periods of time.

There are quite a few similar and likely better tools out there. This is mostly just an excuse to
re-familiarize myself with basic Rust syntax and concepts.

## Usage
```
cargo build --release
OWM_API_KEY=<API key goes here> target/release/wtwr
```

## Todo
- Don't hard-code user preferences or location
- Unit tests
- Host on [AWS Lambda](https://github.com/awslabs/aws-lambda-rust-runtime)
- Make an Alexa skill
