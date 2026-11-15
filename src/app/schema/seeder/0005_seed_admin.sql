INSERT IGNORE INTO `users` (
    `email`,
    `password_hash`,
    `name`,
    `role`,
    `status`,
    `created_at`,
    `updated_at`
) VALUES (
    'admin@galonku.com',
    '$argon2id$v=19$m=19456,t=2,p=1$XbYOPVMneti4Zhf9eKQe9Q$SV01LQhiX2fsalRAj/fCcDO448wpoaILTf++Oc+1rTM',
    'Super Admin',
    'admin',
    'active',
    NOW(),
    NOW()
);
