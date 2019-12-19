CREATE DATABASE acm_manager;
USE acm_manager;
CREATE USER 'acmuser'@'localhost' IDENTIFIED BY 'resumca';
GRANT ALL PRIVILEGES ON acm_manager.* TO 'acmuser'@'localhost';