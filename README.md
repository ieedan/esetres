# esetres

A self hosted file storage server.

## Features

-   Save any file using a rest endpoint
-   Retrieve files using a URL
-   Multiple `buckets`
-   Tokens to manage write access to buckets
    -   Mint, Revoke and List tokens from the CLI

## CLI

### Start

```bash
esetres start

Listening at 0.0.0.0:8080...
```

Run locally

```bash
esetres start --local

Listening at 127.0.0.1:3000...
```

Custom port

```bash
esetres start --port=5000

Listening at 0.0.0.0:5000...
```

### Tokens

Mint

```bash
esetres tokens mint MY_TOKEN

eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9eyJzdWI...
```

List
```bash
esetres tokens list

MY_TOKEN - 6/6/24
```

Revoke

```bash
esetres tokens revoke MY_TOKEN

Token revoked.
```

## Mime Types

The mime types come from [mime-db](https://github.com/jshttp/mime-db). When you first start the server it will grab the types from the db and store it on machine. You can always delete the `mime-db.json` file to refresh it.
