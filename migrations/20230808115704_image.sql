-- Add migration script here
create table if not exists image (
    id serial primary key,
    name varchar(255) not null,
    url varchar(255) not null,
    width int not null,
    height int not null
);




INSERT INTO image ( name, url, width, height ) VALUES ('Image 2', 'https://picsum.photos/200/300', 200, 300);

SELECT * FROM image;