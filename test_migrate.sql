
-- TABLE --: registrator

CREATE TABLE registrator (make_date date, id bigint, code character varying, creator character varying, dec_number character varying, first_use character varying, name character varying, note character varying);;
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '000', 'test', 'ПРГС.123321.112', 'today', 'testtest', 'tt');
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '000', '', 'ПРГС.133223.132', '', 'тест', '');
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '000', '', 'ПРГС.133223.133', '', 'тест', '');
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '000', '', 'ПРГС.133223.134', '', 'тест', '');
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '000', '', 'ПРГС.133223.135', '', 'тест', '');
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '000', '', 'ПРГС.133223.136', '', 'тест', '');
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '000', '', 'ПРГС.133223.137', '', 'тест', '');
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '000', '', 'ПРГС.133442.134', '', 'Система некого транспорта для поддержки жизнедеятельности', '');
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '000', '', 'ПРГС.123321.111', '', 'test', '');
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '000', '', 'ПРГС.123321.113', '', 'test', '');
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '000', '', 'ПРГС.123321.114', '', 'test', '');
INSERT INTO registrator (make_date, id, code, creator, dec_number, first_use, name, note) VALUES (NULL, NULL, '010', 'paul', 'ПРГС.133223.111', 'yesterday', 'test detail', 'test');

-- TABLE --: users

CREATE TABLE users (id bigint, email character varying, password character varying, role character varying, unhashed_password character varying, username character varying);;
INSERT INTO users (id, email, password, role, unhashed_password, username) VALUES (NULL, 'test@test.com', '$2a$10$JHm9G8Cs8fwAJzOrorLpleMoIoCifndLJQIvydyhkL0abkoS.fJHu', 'ROLE_ADMIN', '1234', 'test');

-- TABLE --: orders_copy

CREATE TABLE orders_copy (id integer, date date, count integer, price numeric, customer character varying, product character varying);;
INSERT INTO orders_copy (id, date, count, price, customer, product) VALUES (NULL, NULL, NULL, NULL, 'Цезий Магниевич', 'Радиация');
INSERT INTO orders_copy (id, date, count, price, customer, product) VALUES (NULL, NULL, NULL, NULL, 'Марк Цукенберг', 'React');
