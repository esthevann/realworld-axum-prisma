# axum-prisma-starter

# Usage
You can use [cargo-generate](https://github.com/cargo-generate/cargo-generate) to generate the template
```
  cargo install cargo-generate
  cargo generate --git https://github.com/esthevann/axum-prisma-starter
```

# Setup
1. Make your changes to the schema on ```prisma/schema.prisma```
2. Generate the client 
  ```
  cargo prisma generate
  ```
 3. Push to the db
 ```
 cargo prisma db push
 ```
 
