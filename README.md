# fair-api

This project contains a web service that runs a simplified version of the [FAIR model](https://www.fairinstitute.org/what-is-fair)

## Setup

This is a Rust project. It requires Rust version 1.46 or greater. Additionally, a Dockerfile is supplied that will offer the service without any dependencies beyond Docker.

```sh
cargo run
```

or

```sh
docker build -t fair-api .
docker run -p 8080:8080 fair-api
```

## Interface

The api offers a heartbeat `GET` endpoint at `/` and a risk modeler `POST` endpoint at `/`. And example JSON input is provided in [valid.json](valid.json) 

```sh
curl -s -H "Content-Type: application/json" -d @valid.json http://localhost:8080 | jq .
```

```json
{
  "scenario": {
    "name": "Test Scenario",
    "sample_size": 100000,
    "threat_event_frequency": 0.25,
    "probable_loss_magnitude": 100000,
    "worst_case_loss_magnitude": 1000000,
    "control_strength": {
      "min": 0,
      "max": 100,
      "most_likely": 50
    },
    "threat_capability": {
      "min": 0,
      "max": 100,
      "most_likely": 50
    }
  },
  "probable_loss": 12500.501,
  "worst_case_loss": 125005.01
}
```