![Rusty Shop](SVG/banner.png)
# How to run *rusty-shop* in local

1. install rust in your machine
2. install sqllite and other dependencies
3. install diesel_cli
4. setup database
5. run

----
## 1. install rust on your machine 
best document for install is own [Rust documention](https://www.rust-lang.org/tools/install)

for linux users download and run rust bash file
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
then just add this line to your shell. for bash:
```
echo 'source $HOME/.cargo/env' >> ~/.bashrc
```

## 2. Install sqlite and other dependencies
for use this app and diesel_cli you need to install this packges `sqlite3` , `libsqlite3-0` and `libsqlite3-dev`
in ubuntu with apt packge manager use this code
```
sudo apt install sqlite3 libsqlite3-0 libsqlite3-dev 
```

## 3. Install diesel_cli
just use this command easy && GG
```
cargo install diesel_cli --no-default-features --features sqlite
```


## 4. Setup sqlite Database
after build app you can use diesel to setup database
```
diesel setup --database-url DB/DataBase.SQLITE
```
#### import demo data
use this command to import demo data in database
```
cat DB/demo.sql | sqlite3 DB/DataBase.SQLITE
```


## 5. Finally, Run

just use it!
```
cargo run it!
```
