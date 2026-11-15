# Databse scema sama migration

oke jadi tujuan dari folder `src/app/schema` itu gini ya teman teman

## Latar Belakang

oke jadi ketimbang kita inject `sql string` ke rust code, yang nnti debugginnnya ngak enak sama ngak rapi juga di folder ini tempat semua `query` sql di definisikan, udah itu aja.

## Struktur file

- `mod.rs`

ini gunanya biar rust itu bisa baca semua file `.sql` kayak di fatrarin ke registri kayak gitu lah intinya

- `00X_Foo.sql`

nah kalau file ini itu file sql murni trus berurutan nomronaya, sama kalau udah di deploy jangan di edit filenya, buat baru aja.

## jangan lupa baca note di setiap `.sql` nya y

## Note

nti di file sql itu kan ada kyak gini

`ENGINE=InnoDB`

nah ini senenrya itu jensi storage engine nya mysql sama maria db
kalok masih bingun cek aja di [https://en.wikipedia.org/wiki/InnoDB](https://en.wikipedia.org/wiki/InnoDB) atau di [https://dev.mysql.com/doc/refman/8.4/en/innodb-storage-engine.html](https://dev.mysql.com/doc/refman/8.4/en/innodb-storage-engine.html)

next CHARSET=utf8mb4

ini biar si mysql itu bisa pakek semua huruf di dunia kaya emoji dll



