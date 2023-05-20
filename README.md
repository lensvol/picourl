# picourl

[![License](https://img.shields.io/github/license/lensvol/fl-small-mercies)](https://github.com/lensvol/fl-small-mercies/blob/master/LICENSE)

An experiment in making static URL redirector for my own use.

Heavily inspired by [apoor-dot-dev](https://github.com/a-poor/apoor-dot-dev/) with build time URL inclusion taken almost verbatim from [serde-json-build-example](https://github.com/nicholasfagan/serde-json-build-example).

## Usage

```bash
cargo build
./picourl
```

## Configuration

Redirects are stored in `urls.yaml` file, which has very simple format:

```yaml
- slug: <short id suitable for passing via GET parameter>
  url: <URL>
```

## TODOs

- [ ] Customizable port
- [ ] Configurable fallback URL
- [ ] Support for running on AWS Lambda
- [ ] Customizable 404 page
- [ ] Ping endpoint for health checks

