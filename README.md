📕 BookStore

============
## Configuration

```
mv .env.example .env
```

## Run
````
cargo run
````
---

## Test
````
cargo test
````



## Entity Generation
````
sea generate entity -o src/entities -u mysql://root:@localhost:3306/bookstore
````
