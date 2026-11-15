use db::prelude::Migration;

pub fn migrations() -> Vec<Migration> {
    vec![
        Migration::new(1, "bootstrap", include_str!("0001_bootstrap.sql"))
            .with_down("DROP TABLE IF EXISTS `_bootstrap`"),
        Migration::new(2, "users", include_str!("0002_users.sql"))
                    .with_down("DROP TABLE IF EXISTS `users`"),
        Migration::new(3, "roles", include_str!("0003_roles.sql"))
                    .with_down("DROP TABLE IF EXISTS `roles`"),
        Migration::new(4, "seed_roles", include_str!("seed/0004_seed_roles.sql"))
                   .with_down("TRUNCATE TABLE `roles`"),
    ]
}
