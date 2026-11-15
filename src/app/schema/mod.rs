use db::prelude::Migration;

pub fn migrations() -> Vec<Migration> {
    vec![
        Migration::new(1, "bootstrap", include_str!("system/0001_bootstrap.sql"))
            .with_down("DROP TABLE IF EXISTS `_bootstrap`"),
        Migration::new(2, "users", include_str!("master/0002_users.sql"))
            .with_down("DROP TABLE IF EXISTS `users`"),
        Migration::new(6, "addresses", include_str!("master/0006_addresses.sql"))
            .with_down("DROP TABLE IF EXISTS `addresses`"),
        Migration::new(8, "mitras", include_str!("master/0008_mitras.sql"))
            .with_down("DROP TABLE IF EXISTS `mitras`"),
        Migration::new(9, "agents", include_str!("master/0009_agents.sql"))
            .with_down("DROP TABLE IF EXISTS `agents`"),
        Migration::new(3, "roles", include_str!("enums/0003_roles.sql"))
            .with_down("DROP TABLE IF EXISTS `roles`"),
        Migration::new(
            10,
            "application_statuses",
            include_str!("enums/0010_application_statuses.sql"),
        )
        .with_down("DROP TABLE IF EXISTS `application_statuses`"),
        Migration::new(4, "seed_roles", include_str!("seeder/0004_seed_roles.sql"))
            .with_down("TRUNCATE TABLE `roles`"),
        Migration::new(5, "seed_admin", include_str!("seeder/0005_seed_admin.sql"))
            .with_down("DELETE FROM `users` WHERE `email` = 'admin@galonku.com'"),
        Migration::new(
            11,
            "seed_application_statuses",
            include_str!("seeder/0011_seed_application_statuses.sql"),
        )
        .with_down("TRUNCATE TABLE `application_statuses`"),
        Migration::new(
            7,
            "agent_applications",
            include_str!("transaction/0007_agent_applications.sql"),
        )
        .with_down("DROP TABLE IF EXISTS `agent_applications`"),
    ]
}
