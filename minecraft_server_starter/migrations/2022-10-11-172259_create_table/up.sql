pragma encoding=UTF8;
CREATE TABLE `users` (
  `id` int AUTO_INCREMENT PRIMARY KEY,
  `username` text NOT NULL,
  `password` text NOT NULL,
  `user_type` smallint NOT NULL
);