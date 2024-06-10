# esetres

A self hosted file storage server.

## Setup

### Environment Variables

Create a .env file and configure the following variables.

- `TOKEN_SECRET` - The secret for jwt encoding (should be strong)
- `IP` - The ip address of the server use `127.0.0.1` for local and the ip of the machine when hosting. **IMPORTANT** if you use 0.0.0.0 when hosting the server will still run but the cache will not refresh when changing tokens.
- `PORT` - The port the server runs on
- `HTTPS` - Specifies whether or not to use https in the url (1 for yes anything else for no)

Example .env

```js
TOKEN_SECRET="secret"

IP="127.0.0.1"
PORT=3000
HTTPS=0
```

### Run Migration

esetres uses a local sqlite database to store the tokens so you will need to run the migration.

```bash
esetres migrate
```

### Start the server

```bash
esetres start
```

## API

> [!NOTE]
> Clients may be added in the future

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

Starts the server.

```bash
esetres start

Listening at 127.0.0.1:3000...
```

### Migrate

Creates the database and necessary tables.

```bash
esetres migrate
```

### Tokens

Tokens allow you to control access to the server. It uses jwt with the Bearer scheme meaning authorized routes require the authorization header.

Tokens are cached by the server for fast responses and when you create or delete tokens the cache will automatically be updated.

#### Mint

Creates a new token with specified scope and access.

Create

```bash
esetres tokens mint MY_TOKEN

New token (MY_TOKEN) created for scope (*).
eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

Set **Scope**

```bash
# default scope is "*"
esetres tokens mint MY_TOKEN --scope default
```

Set **Access**

```bash
# default access is "read"
esetres tokens mint MY_TOKEN --access write
```

#### List

Lists all existing tokens with the most recently created being at the top.

```bash
esetres tokens list

Name                        Scope     Access
----------------------------------------------
ANOTHER_REALLY_LONG_TOKEN | test    | full
NEW_TEST_TOKEN            | default | read
TEST_TOKEN                | *       | write
```

#### Revoke

Deletes a token from the database.

```bash
esetres tokens revoke MY_TOKEN

Token revoked.
```

## Mime Types

The mime types come from [mime-db](https://github.com/jshttp/mime-db). When you first start the server it will grab the types from the db and store it on machine. You can always delete the `mime-db.json` file to refresh it.
