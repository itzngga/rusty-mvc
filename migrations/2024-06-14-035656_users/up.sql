-- Your SQL goes here
CREATE TABLE `users`(
	`id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	`username` TEXT NULL,
	`password` TEXT NULL,
	`created_at` TIMESTAMP NULL,
	`updated_at` TIMESTAMP NULL
);

