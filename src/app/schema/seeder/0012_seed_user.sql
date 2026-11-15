INSERT INTO `users` (
    `email`,
    `password_hash`,
    `name`,
    `role`,
    `status`,
    `created_at`,
    `updated_at`
) VALUES
(
    'customer@galonku.com',
    '$argon2id$v=19$m=19456,t=2,p=1$d+2CN1QNvR4XaSYS1WqvYw$4vb7Lfkpe3qb7THbRyKASfN6CScWM7aAIsD0B69OmhQ',
    'Budi Konsumen',
    'customer',
    'active',
    NOW(),
    NOW()
),
(
    'agent@galonku.com',
    '$argon2id$v=19$m=19456,t=2,p=1$d+2CN1QNvR4XaSYS1WqvYw$4vb7Lfkpe3qb7THbRyKASfN6CScWM7aAIsD0B69OmhQ',
    'Andi Agent',
    'agent',
    'active',
    NOW(),
    NOW()
);
