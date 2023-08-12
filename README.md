# Configmonkey

A lightweight configuration service that supports managing app configurations across multiple environments. Inspired by tools like [etcd](https://etcd.io/), [consul kv](https://developer.hashicorp.com/consul/docs/dynamic-app-config/kv) and [spring cloud config](https://docs.spring.io/spring-cloud-config/docs/current/reference/html/), Configmonkey was built with distributed environments in mind, and it's goal is to provide an easy way to externalize and manage microservice configurations from a single place, without interfering with the application's ci/cd lifecycle.

## Table of Contents

- [Configmonkey](#configmonkey)
  - [Table of Contents](#table-of-contents)
  - [How to run](#how-to-run)
    - [Local development](#local-development)
    - [Running the tests](#running-the-tests)
  - [Contributing](#contributing)

## How to run

> ⚠️ Configmonkey is still under development and might not be stable under load. We don't recommend using it in production at this stage.

### Local development

Clone this repo and start the dependencies with `docker-compose`:

```shell
docker-compose up
```

Alternatively, if you prefer to start the dependencies in a different way (postgres database), adjust the configuration in `.cargo/config.toml` with the appropriate connection strings.

Then start the app using `cargo`:

```shell
cargo run
```

If everything started correctly, the application will then be available at http://127.0.0.1:8000.

### Running the tests

With the dependencies already started, run the tests using `cargo`. The test suite creates several databases and requires the environment variable `DATABASE_URL` to point to any postgres database. Check `.cargo/config.toml` if you want to use a different database for tests.

```shell
cargo test
```

## Contributing

[![Contributors](https://img.shields.io/github/contributors/madoke/configmonkey)](https://github.com/madoke/configmonkey/graphs/contributors) [![Commits](https://img.shields.io/github/commit-activity/m/madoke/configmonkey)](https://github.com/madoke/configmonkey/graphs/contributors)

Configmonkey is a recent project and welcomes new contributors. The preferred ways to help out are:

- Opening an issue reporting a problem or feature creep;
- Submit a pull request to fix an open issue;
- Submit a pull request to fix a bug that doesn't have an open issue;
