create extension if not exists "uuid-ossp";

create table subscription_type(
  id serial primary key not null,
  name text not null,
  monthly_recurring_revenue_cents int not null
);

insert into subscription_type (name, monthly_recurring_revenue_cents) values
    ('initializing', 0),
    ('basic plan', 500),
    ('free', 0),
    ('unsubscribed', 0),
    ('free trial', 0)
;

create table users(
    id serial primary key,
    created_at timestamp with time zone not null default now(),
    digest varchar(255) not null,
    email varchar(255) unique not null,
    salt varchar(255) not null,
    stripe_customer_id varchar(255) not null,
    subscription_type_id int not null references subscription_type(id) default 3,
    username varchar(255) unique not null
);

create table password_reset_link(
    user_id int not null primary key references users(id),
    created_at timestamp with time zone not null default now(),
    slug text not null
);

create table audit_stripe_webhooks(
    id serial primary key not null,
    payload text not null,
    created_at timestamp with time zone not null default now(),
    includes_usable_update boolean not null
);

