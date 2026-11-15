CREATE TABLE IF NOT EXISTS `agent_applications` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `user_id` BIGINT NOT NULL,
    `status` VARCHAR(20) NOT NULL DEFAULT 'pending',
    `applied_at` TIMESTAMP NULL,
    `reviewed_at` TIMESTAMP NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `uniq_agent_app_user` (`user_id`),
    CONSTRAINT `fk_agent_app_user` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
