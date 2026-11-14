use db::prelude::Migration;

pub fn migrations() -> Vec<Migration> {
    vec![
        Migration::new(1, "bootstrap", include_str!("0001_bootstrap.sql"))
            .with_down("DROP TABLE IF EXISTS `_bootstrap`"),
    ]
}
