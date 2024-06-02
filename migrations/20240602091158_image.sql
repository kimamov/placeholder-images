-- Add migration script here
create table if not exists image (
    id serial primary key,
    name varchar(255) not null,
    url varchar(255) not null,
    width int not null,
    height int not null,
    thumbnail varchar(255) not null
);