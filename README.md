# Quickstart
You can use this tool to create new RocksDB databases manually
```
cargo run my_database.rocksdb
rocksdb> count
Total number of keys: 0
rocksdb> put 1 10
rocksdb> put 2 20
rocksdb> put 3 30
rocksdb> count
Total number of keys: 3
rocksdb> scan
All keys in the database:
1
2
3
rocksdb> get 2
20
rocksdb> put 2 60
rocksdb> get 2
60
rocksdb> q
```

Or you can interact with existing databases

```
cargo run my_database.rocksdb
rocksdb> count
Total number of keys: 3
rocksdb> scan
All keys in the database:
1
2
3
rocksdb> get 2
60
rocksdb> q
```
