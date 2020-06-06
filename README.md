![Rusty Shop](SVG/banner.png)
# How to run *rusty-shop* in local

1- install rust in your machine

2- install sqllite and other dependencies

3- install diesel_cli

4- build wit cargo

5- setup database

6- run

----
## install rust on your machine 
best document for install is own [Rust documention](https://www.rust-lang.org/tools/install)

for linux users download and run rust bash file
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
then just as shell say run
```
source $HOME/.cargo/env
```

## Install sqlite and other dependencies
for use this app and diesel_cli you need to install this packges `sqlite3` , `libsqlite3-0` and `libsqlite3-dev`
in ubuntu with apt packge manager use this code
```
sudo apt install sqlite3 libsqlite3-0 libsqlite3-dev 
```

## Install diesel_cli
just use this command easy && GG
```
cargo install diesel_cli --no-default-features --features sqlite
```


## complie app
for complie use can use cargo with that command
```
cargo build
```

### Setup sqlite Database
after build app you can use diesel to setup database
```
diesel setup --database-url DB/DataBase.SQLITE
```
 *seed the database* 
for seed database use this command to import defualt value in database
```
cat DB/demo.sql | sqlite3 DB/DataBase.SQLITE

```


### Finally, Run

just use
```
cargo run it!
```
