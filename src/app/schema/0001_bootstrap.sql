CREATE TABLE IF NOT EXISTS `_bootstrap` (
    `id` TINYINT NOT NULL PRIMARY KEY,
    `created_at` TIMESTAMP NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- oke jadi simpelnya file ini itu kayak gini,, ini tujuanya biar mastiin kalau koneksi sama migrasi itu sukses atau ngak sebenernya. udh itu aja
-- tujuan field field nya,,
-- 
-- id : nah kalok kita tau sbeneya di mysql ini kita kan wajib buat pk di tiap table ya
-- create_at : nah ini sbennynya gini,,
-- di dependensi db = { git = "https://github.com/mroczect/pbl-trpl-106-databases", branch = "main", version = "0.1.0" }
-- ini kmrn waktu aku di debug ada maslaha kyak gini,, `failed to decode column` nah jadi tujuanya cuman buat negtes kira kira kalau `TIMESTAMP NULL` ini dia bakal eror atau ngak itu aja si benryna