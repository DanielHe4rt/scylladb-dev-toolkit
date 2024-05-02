# ScyllaDB Daniel Dev Toolkit

This is just probably a messy project to create things to make my work around ScyllaDB and Rust environment easier.

> Don't expect great things from this code - Daniel Reis

## How to use

Select an action from the list below and run the command on the terminal:

* keyspace
* multi-dc (WIP)

````bash
toolkit --action keyspace --keyspace he4rtless_ks
````

## CLI Arguments

| Argument | Required | Default        |
|----------|----------|----------------|
| action   | yes      |                |
| host     | no       | localhost:9042 |
| keyspace | no       | mykeyspace     |
| user     | no       | -empty-        |
| password | no       | -empty-        |
| timeout  | no       | 30             |

## Features

- [x] Create a new keyspace without authentication on CQLSH
- [ ] Alter system tables to Multi-DC
- [ ] Setup multiple Schemas for testing purposes.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details