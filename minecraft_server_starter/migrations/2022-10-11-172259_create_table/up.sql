pragma encoding=UTF8;
CREATE TABLE `users` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `username` text NOT NULL,
  `password` text NOT NULL,
  `user_type` smallint NOT NULL
);

CREATE TABLE `sessions` ( 
  `id` int AUTO_INCREMENT PRIMARY KEY, 
  `expiration` DATETIME NOT NULL, 
  `user_id` int NOT NULL 
);