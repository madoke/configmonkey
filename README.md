# Configmonkey

A lightweight configuration service that supports managing app configurations across multiple environments. Inspired by tools like [etcd](https://etcd.io/), [consul kv](https://developer.hashicorp.com/consul/docs/dynamic-app-config/kv) and [spring cloud config](https://docs.spring.io/spring-cloud-config/docs/current/reference/html/), Configmonkey was built with distributed environments in mind, and it's goal is to provide an easy way to externalize and manage microservice configurations from a single place, without interfering with the application's ci/cd lifecycle.

## Table of Contents

- [How to Run](#how-to-run)
- [API Reference](#api-reference)
- [Contributing](#contributing)

## How To Run

### App on host with dependencies on docker

Recommended for local development

```
# Start Dependencies
docker compose --profile deps-only up

# Start Configmonkey
cargo run
```

### Full docker setup

Recommended for deployments/general usage

```
docker compose --profile full up
```

## API Reference

### `/v1/apps`

#### Create a new app

- Url: `/v1/apps`
- Method: `POST`
- Params:
  - name : The application name
  - slug: The application slug (to be used in urls)
- Response fields:
  - name : The application name
  - slug: The application slug (to be used in urls)
  - created_at: The application's created date
  - updated_at: The application's updated date

##### Example Request

```json
POST /v1/apps
{
    "slug": "configmonkey",
    "name": "Config Monkey"
}
```

##### Example Response

```json
201 CREATED
{
  "slug": "configmonkey",
  "name": "Config Monkey",
  "created_at": "2023-08-11T23:50:00Z",
  "updated_at": "2023-08-11T23:50:00Z"
}
```

#### List existing apps

- Url: `/v1/apps`
- Method: `GET`
- Params:
  - limit: Max number of applications to retrieve (default: 10)
  - offset: Number of applications to skip (default: 0)
- Response fields:
  - data : The applications
  - pagination: Pagination object to navigate the full list of applications
    - next: URI path for next page
    - prev: URI path for previous page
    - limit: Max applications in current page
    - offset: Number of applications skipped
    - count: Total applications in current page

##### Example Request

```json
GET /v1/apps?limit=1&offset=1
```

##### Example Response

```json
200 OK
{
  "data": [
    {
      "slug": "configmonkey",
      "name": "Config Monkey",
      "created_at": "2023-08-11T23:50:00Z",
      "updated_at": "2023-08-11T23:50:00Z"
    }
  ],
  "pagination": {
    "count": 1,
    "limit": 1,
    "offset": 1,
    "prev": "/v1/apps?limit=1&offset=0",
    "next": "/v1/apps?limit=1&offset=2"
  }
}
```

#### Delete an existing app

- Url: `/v1/apps/{app_slug}`
- Method: `DELETE`
- Params:
  - app_slug: The slug of the app to delete
- Response fields: N/A

##### Example Request

```json
DELETE /v1/apps/configmonkey
```

##### Example Response

```json
204 NO CONTENT
```

### `/v1/envs`

#### Create a new env

- Url: `/v1/envs/{app_slug}`
- Method: `POST`
- Params:
  - app_slug: The slug of the application to add the environment to
  - name : The environment name
  - slug: The environment slug (to be used in urls)
- Response fields:
  - name : The environment name
  - slug: The environment slug (to be used in urls)
  - created_at: The environment's created date
  - updated_at: The environment's updated date

##### Example Request

```json
POST /v1/envs/configmonkey
{
    "slug": "staging",
    "name": "Staging"
}
```

##### Example Response

```json
201 CREATED
{
  "slug": "staging",
  "name": "Staging",
  "created_at": "2023-08-11T23:50:00Z",
  "updated_at": "2023-08-11T23:50:00Z"
}
```

#### List existing envs

- Url: `/v1/envs/{app_slug}`
- Method: `GET`
- Params:
  - app_slug: The slug of the application to list environments from
  - limit: Max number of environments to retrieve (default: 10)
  - offset: Number of environments to skip (default: 0)
- Response fields:
  - data : The environments
  - pagination: Pagination object to navigate the full list of environments
    - next: URI path for next page
    - prev: URI path for previous page
    - limit: Max environments in current page
    - offset: Number of environments skipped
    - count: Total environments in current page

##### Example Request

```json
GET /v1/envs/configmonkey?limit=1&offset=1
```

##### Example Response

```json
200 OK
{
  "data": [
    {
      "slug": "staging",
      "name": "Staging",
      "created_at": "2023-08-11T23:50:00Z",
      "updated_at": "2023-08-11T23:50:00Z"
    }
  ],
  "pagination": {
    "count": 1,
    "limit": 1,
    "offset": 1,
    "prev": "/v1/envs?limit=1&offset=0",
    "next": "/v1/envs?limit=1&offset=2"
  }
}
```

#### Delete an existing env

- Url: `/v1/envs/{app_slug}/{env_slug}`
- Method: `DELETE`
- Params:
  - app_slug: The slug of the application containing the environment
  - env_slug: The environment's slug
- Response fields: N/A

##### Example Request

```json
DELETE /v1/envs/configmonkey/staging
```

##### Example Response

```json
204 NO CONTENT
```

### `/v1/configs`

## Contributing

[![Contributors](https://img.shields.io/github/contributors/madoke/configmonkey)](https://github.com/madoke/configmonkey/graphs/contributors) [![Commits](https://img.shields.io/github/commit-activity/m/madoke/configmonkey)](https://github.com/madoke/configmonkey/graphs/contributors)

Configmonkey is a recent project and welcomes new contributors. The preferred ways to help out are:

- Opening an issue reporting a problem or feature creep;
- Submit a pull request to fix an open issue;
- Submit a pull request to fix a bug that doesn't have an open issue;

Detailed information about how to contribute can be found in the [contribution guide](CONTRIBUTING.md)
