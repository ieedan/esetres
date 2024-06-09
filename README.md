# esetres

A self hosted file storage server.

## Features

-   Save any file using a rest endpoint
-   Retrieve files using a URL
-   Multiple `buckets`
-   Tokens to manage write access to buckets
    -   Mint, Revoke and List tokens from the CLI

## Getting started

### Environment Variables

Create a .env file and add the `TOKEN_SECRET` variable. This will be used for the jwt encoding.

Example .env
```
TOKEN_SECRET="secret"
```

## API

### Health Check `GET/health`
Returns a simple running status for the server.

### Create bucket `POST/buckets` `(authed)`
Creates a new bucket.

Request Body:
```json
{
    "name": "bucket name"
}
```

### Cache Invalidate `POST/cache/invalidate`
This is configured for local only access. It will invalidate the token cache causing a refetch from the database. This will be hit when you mint or revoke tokens.

### Get File `GET/buckets/{bucket}/blob/{file_name}`
Gets the requested file from the requested bucket.

### Upload File `PUT/buckets/{bucket}/blob/{file_name}` `(authed)`
Uploads the file to the requested bucket with the requested file name.

Request Body: File Content In Bytes

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
