# crcl


## Running migrations

Running the server will attempt to run migrations by default. If you want to run migrations manually, you can do so in the migration crate:

```bash
cd migrations
cargo run -- up
```

And likewise, to down them:

```bash
cargo run -- down
```

To generate the models after migrating, run this in the root of the project:

```bash
sea-orm-cli generate entity -o entity/src/entities --with-serde both
```

## Running the server

To run the server, you can use the following command:

```bash
cargo run
```
