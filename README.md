# ScyllaDB Daniel Dev Toolkit

This is just probably a messy project to create things to make my workaround ScyllaDB and Rust environment easier.

> Don't expect great things from this code - Daniel Reis

## How to use

Select an action from the list below and run the command on the terminal:

* keyspace
* multi-dc (WIP)
* table-action

## Features

### Create a new keyspace without authentication on CQLSH

A quick way to set up a Keyspace with a given name and replication factor. Also supports flags for tablets in case you
need to use features which is not supported by Tablets yet (e.g: LWT and CDC):

```markdown
| Option                                      | Description                                | Default             |
|---------------------------------------------|--------------------------------------------|---------------------|
| `--host <HOST>`                             | Scylla Host                                | `localhost:9042`    |
| `-u, --user <USER>`                         |                                            |                     |
| `-p, --password <PASSWORD>`                 |                                            |                     |
| `-t, --timeout <TIMEOUT>`                   |                                            | `3`                 |
| `-k, --keyspace <KEYSPACE>`                 | Keyspace name                              | `mykeyspace`        |
| `-r, --replication-factor <REPLICATION_FACTOR>` | Replication factor                        | `3`              |
| `-d, --drop`                                | Drop the keyspace if it already exists     |                     |
| `--without-tablets`                         |                                            |                     |
| `-h, --help`                                | Print help                                 |                     |
```

```bash
toolkit keyspace --keyspace he4rtless_ks --replication-factor 1 --without-tablets 
```

### Truncate or Drop specific tables with suffix

This feature is useful when you need to clean up your database from tables with a specific suffix.

```markdown
| Option                                      | Description                                | Default             |
|---------------------------------------------|--------------------------------------------|---------------------|
| `--host <HOST>`                             | Scylla Host                                | `localhost:9042`
| `-u, --user <USER>`                         |                                            |                     |
| `-p, --password <PASSWORD>`                 |                                            |                     |
| `-t, --timeout <TIMEOUT>`                   |                                            | `3`                 |
| `-k, --keyspace <KEYSPACE>`                 | Keyspace name                              | `mykeyspace`        |
| `-s, --suffix <SUFFIX>`                     | Suffix to filter tables                    | `test`              |
| `-a, --action`                              | If you want to 1-truncate or 2-delete      |                     |
| `-h, --help`                                | Print help                                 |                     |
```

```bash
toolkit truncate --keyspace he4rtless_ks --suffix v1 --action 1
```

### Setup multiple Schemas for testing purposes.

A simple way to alter tables which is in an already created ScyllaDB Cluster and enable it to be Multi-DC.


```markdown
| Option                                      | Description                                | Default             |
|---------------------------------------------|--------------------------------------------|---------------------|
| `--host <HOST>`                             | Scylla Host                                | `localhost:9042`    |
| `-u, --user <USER>`                         |                                            |                     |
| `-p, --password <PASSWORD>`                 |                                            |                     |
| `-t, --timeout <TIMEOUT>`                   |                                            | `3`                 |
| `-r, --replication-factor <RF>`             | Replication factor                         | `3`                 |
| `-d, --dcs <DCS>`                           | Data centers (comma-separated)             | `DC1,DC2`           |
| `-k, --keyspace <KEYSPACE>`                 | Keyspace name                              | `mykeyspace`        |
| `-h, --help`                                | Print help                                 |                     |

```

```bash
toolkit multi-dc --keyspace he4rtless_ks --replication-factor 3 --dcs DC1,DC2
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details