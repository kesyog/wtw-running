# 🎽 What to wear running
Recommends a running outfit for you based on weather conditions partially pulled from OpenWeatherMap
and user preferences.

Gear data ripped off from a flaky [Runner's World page](https://www.runnersworld.com/training/a20803133/what-to-wear/)
that tends to spontaneously disappear for periods of time.

There are quite a few similar and likely better tools out there. This is mostly just an excuse to
re-familiarize myself with basic Rust syntax and concepts.

## Usage
```
OWM_API_KEY=<API key goes here> cargo run --release
```

## Todo
Super WIP

- Resolve weather conditions from OpenWeatherMap data
- Don't hard-code user preferences or location
- Tests
- Host on [AWS Lambda](https://github.com/awslabs/aws-lambda-rust-runtime)
- Make an Alexa skill
