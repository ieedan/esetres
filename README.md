# esetres

A self hosted file storage server.

## Setup

### Install

```bash
cargo install esetres

# check version to verify installation
esetres -V
```

### Run Init

Run the [init](#init) command and go through the setup process.

```bash
esetres init
```

### Start the server

```bash
esetres start
```

> [!TIP]
> See our [examples](https://github.com/ieedan/esetres/tree/main/examples) to see how to host and structure your server.

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
	"name": "bucket_name"
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

### Init

Step by step process for setting up the environment variables, running the migration and creating your first bucket.

```bash
esetres init

Welcome to esetres cli!
|
o Do you want us to generate the token secret? yes
|
✓ Generated token secret.
|
o Select your the ip: 172.31.144.1
|
o Enter the port: 8080
|
o Use https? no

TOKEN_SECRET="[hidden]"

IP="172.31.144.1"
PORT="8080"
HTTPS=0

o .env file Ok? yes
|
✓ Created .env file.
|
o Run sqlite migration? yes
|
✓ Ran sqlite migration.
|
o Create a bucket? yes
|
o Enter the bucket name: default
|
✓ Bucket [default] created.
|
✓ Completed initialization.
```

### Start

Starts the server.

```bash
esetres start

Listening at 127.0.0.1:3000...
```

### Run Migration

esetres uses a local sqlite database to store the tokens. In the `init` function you can choose to run this automatically or you can run it yourself at any time with:

```bash
esetres migrate
```

### Buckets

You can create buckets from the api or by creating the folders yourself on the server. However the CLI enables some extra functionality.

#### Create

Creates a new bucket and the required folders.

```bash
esetres buckets create my_bucket
```

#### Delete

Deletes the bucket and all contents.

```bash
esetres buckets delete my_bucket
```

#### List

Lists all the buckets and their last modified time.

```bash
esetres buckets list

Name      Modified
-----------------------------------------------
default | 2024-06-11 09:36:37.522380 -05:00
test    | 2024-06-11 09:44:01.888576100 -05:00
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

Scope allows you to limit the buckets a token has access to.

```bash
# default scope is "*"
esetres tokens mint MY_TOKEN --scope default
```

Set **Access**

Access allows you to limit the permissions a token has to create buckets, upload files, and read files.

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
