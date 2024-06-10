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

```js
# Should be a strong secret for jwt encoding
TOKEN_SECRET="secret"

IP="127.0.0.1"
PORT=3000
# Determines whether or not to use http or https in the url
HTTPS=0
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

### Invalidate Cache `POST/cache/invalidate`

This is configured for local only access. It will invalidate the token cache causing a refetch from the database. This will be hit when you mint or revoke tokens.

### Get File `GET/buckets/{bucket}/{public | private}/{file_name}`

Gets the requested file from the requested bucket. Requests to `private` must be authorized.

### Upload File `PUT/buckets/{bucket}/{public | private}/{file_name}` `(authed)`

Uploads the file to the requested bucket with the requested file name.

Request Body: File Content In Bytes

## CLI

### Start

```bash
esetres start

Listening at 127.0.0.1:3000...
```

### Tokens

#### Mint

Create

```bash
esetres tokens mint MY_TOKEN

New token (MY_TOKEN) created for scope (*).
eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

Set **Scope**

```bash
esetres tokens mint MY_TOKEN --scope default
```

Set **Access**

```bash
esetres tokens mint MY_TOKEN --access write
```

#### List

```bash
esetres tokens list

Name                        Scope     Access  
----------------------------------------------
ANOTHER_REALLY_LONG_TOKEN | test    | full    
NEW_TEST_TOKEN            | default | read    
TEST_TOKEN                | *       | write    
```

#### Revoke

```bash
esetres tokens revoke MY_TOKEN

Token revoked.
```

## Mime Types

The mime types come from [mime-db](https://github.com/jshttp/mime-db). When you first start the server it will grab the types from the db and store it on machine. You can always delete the `mime-db.json` file to refresh it.
