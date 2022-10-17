pragma encoding=UTF8;
CREATE TABLE `users` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `username` text NOT NULL,
  `password` text NOT NULL,
  `user_type` smallint NOT NULL
);

CREATE TABLE `sessions` ( 
  `id` INTEGER PRIMARY KEY AUTOINCREMENT, 
  `expiration` DATETIME NOT NULL, 
  `user_id` int NOT NULL 
);