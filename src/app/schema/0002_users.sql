CREATE TABLE IF NOT EXISTS `users` (
    `id` BIGINT NOT NULL AUTO_INCREMENT, 
    `email` VARCHAR(254) NOT NULL,
    `password_hash` VARCHAR(255) NOT NULL,
    `name` VARCHAR(255) NOT NULL,
    `role` VARCHAR(20) NOT NULL DEFAULT 'customer',
    `status` VARCHAR(20) NOT NULL DEFAULT 'active',
    `created_at` TIMESTAMP NULL,
    `updated_at` TIMESTAMP NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uniq_users_email` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ini aku pakai auto increment biar konsisten aja nnti, sama biarenak aja. 
-- 
-- `role` VARCHAR(20) NOT NULL DEFAULT 'customer'
-- nah ini tujuanya itu cuman kalau ada yang regist dia itu langsung jadi costumer aja biar ngak ada nnti yang ngak ada role, nah trus juga kan kalau mislanya dia mau jadi agent kan dia regist jadi agent juag trus disitu paun ada approva nya jg
-- 
-- `status` VARCHAR(20) NOT NULL DEFAULT 'active',
-- nah kalok ini gnanya cuman buat akun ini maish aktif atay ngak, kayak bsia on atau off aja.
-- 
-- waa ngak sesuai laporan https://polnebat-my.sharepoint.com/:w:/r/personal/4342611034_students_polibatam_ac_id/_layouts/15/doc2.aspx?sourcedoc=%7BD486F0AA-FAA6-48A7-8158-F99FDC6E55EF%7D&file=Format%20laporan%20(6).docx&action=default&mobileredirect=true&wdPreviousSession=96319708-7971-406c-998f-ce0c653ce90f&wdPreviousSessionSrc=AppHomeWeb&usid=c3927c82-ae5c-4a27-990b-e0324666aa7 ya tapi oke lah