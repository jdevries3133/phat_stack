create table users(
    id serial primary key,
    created_at timestamp with time zone not null default now(),
    digest varchar(255) not null,
    email varchar(255) unique not null,
    salt varchar(255) not null,
    username varchar(255) unique not null
);

create table password_reset_link(
    user_id int not null primary key references users(id),
    created_at timestamp with time zone not null default now(),
    slug text not null
);
