# Configmonkey

[![Contributors](https://img.shields.io/github/contributors/madoke/configmonkey)](https://github.com/madoke/configmonkey/graphs/contributors)
[![Commits](https://img.shields.io/github/commit-activity/m/madoke/configmonkey)](https://github.com/madoke/configmonkey/graphs/contributors)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/:madoke/:configmonkey/:main)

A lightweight configuration service that supports managing app configurations across multiple environments. Inspired by tools like [etcd](https://etcd.io/), [consul kv](https://developer.hashicorp.com/consul/docs/dynamic-app-config/kv) and [spring cloud config](https://docs.spring.io/spring-cloud-config/docs/current/reference/html/), Configmonkey was built with distributed environments in mind, and it's goal is to provide an easy way to externalize and manage microservice configurations from a single place, without interfering with the application's ci/cd lifecycle.

**Table of Contents**

- [Configmonkey](#configmonkey)
  - [How to run](#how-to-run)
    - [Local development](#local-development)
    - [Full stack](#full-stack)
  - [Building](#building)
  - [Running the tests](#running-the-tests)
  - [API Reference](#api-reference)
  - [Contributing](#contributing)
    - [Issues](#issues)
    - [Pull requests](#pull-requests)
    - [Building a client](#building-a-client)
  - [Next Steps](#next-steps)
    - [Single and nested config support](#single-and-nested-config-support)
    - [Authentication](#authentication)
    - [Multi tenant support](#multi-tenant-support)
    - [Multiple file format](#multiple-file-format)

## How to run

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

### Full stack

{% note %}

**Note:** Configmonkey is still under development and is not stable for production use. At a later stage we will release a proper guide on how to install configmonkey on a traditional container setup

{% endnote %}

## Building

```shell
cargo build
```

Note that the build process doesn't include running the tests. Use this command if you just want to make sure the code compiles properly and are not interested in running the app and the dependencies

## Running the tests

With the dependencies already started, run the tests using `cargo`. The test suite creates several databases and requires the environment variable `DATABASE_URL` to point to any postgres database. Check `.cargo/config.toml` if you want to use a different database for tests.

```shell
cargo test
```

## API Reference

Run an http server from the `docs` folder or head to [https://madoke.github.io/configmonkey/#/](https://madoke.github.io/configmonkey/#/)

## Contributing

Configmonkey is a recent project and welcomes new contributors. Since this is still at an early stage of maturity, there are plenty of ways to contribute

### Issues

We'd love to hear about your experience with configmonkey and get feedback on how it could be improved. At this stage, we're interested in all kinds of issues:

- General feedback on the overall experience (please use it !)
- Bug reporting
- Feature requests or suggestions

### Pull requests

Code contributions are certainly welcome. If this will be a large PR/contribution, please make sure to open an issue first so that we can disscuss it. Eitherway, we'll pay attention to all kinds of PRs:

- Fixing an open issue
- Fixing a bug
- Implementing a new feature (Let's discuss first)
- Updating documentation
- Implementing tests

### Building a client

Currently the only interface to configmonkey is via the REST API, which means that users will need to build the integrations on their side. While we have plans to add several clients in the future, any kinf of contribution here is welcome:

- CLI tool
- Node SDK
- Rust SDK
- Golang SDK
- Java SDK
- ...

If you decide to build one of these, make sure you let us know, so that we can reference it in the docs!

## Next Steps

At the moment, configmonkey is a pet project and therefore we don't have any kind of deadlines nor a roadmap. Contributors are welcome to open discussions (use the issues for that) and help shape the future of this tool. There are however, some items we're thinking about focusing next:

### Single and nested config support

Right now, there is no support for editing or retrieving single and nested config properties, restricting the `/v1/configs` to editing full configuration maps. We want to extend this endpoint to make it as simple and flexible as possible

### Authentication

The only way to run configmonkey now is unauthenticated, which may be enough for most systems, but we want to have the option of extra security by adding JWT validation as a configurable option

### Multi tenant support

Initially thought of, tenant support is hidden under the default tenant in the database. We can leverage the authentication feature to segregate applications, environments and configs across multiple tenants

### Multiple file format

The current format exported by the API is JSON, which will cover most use cases, but we can make use of the `Accept` header to export the configurations on other popular formats like `.env`, `toml`, `yml` etc
