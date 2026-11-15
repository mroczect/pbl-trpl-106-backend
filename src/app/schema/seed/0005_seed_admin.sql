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
    '$argon2id$v=19$m=19456,t=2,p=1$1ZMmPmCX8X440rZh850Now$atDlctqhrAWNXYsUr332+7+v2Hc+7zG/wgdTPsHwe4E',
    'Super Admin',
    'admin',
    'active',
    NOW(),
    NOW()
);
