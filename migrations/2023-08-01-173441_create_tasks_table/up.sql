-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tasks (
    id serial primary key not null,
    title varchar(255) not null,
    completed boolean not null
);