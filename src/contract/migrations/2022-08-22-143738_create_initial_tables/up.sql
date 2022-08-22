-- Your SQL goes here
create table purchases (
    id varchar(250) primary key,
    userid varchar(250) not null,
    product varchar(250) not null,
    shop varchar(250) not null,
    timestamp timestamp not null
);

create index ix_purchases_user on purchases (
    userid,
    timestamp desc
);

create table wishlist (
    id varchar(250) primary key,
    userid varchar(250) not null,
    product varchar(250) not null,
    timestamp timestamp not null
);

create index ix_wishlist_user on wishlist (
    userid
);

create table favoriteProducts (
    id varchar(250) primary key,
    userid varchar(250) not null,
    product varchar(250) not null,
    timestamp timestamp not null
);

create index ix_favoriteProducts_user on favoriteProducts (
    userid
);

create table favoriteShops (
    id varchar(250) primary key,
    userid varchar(250) not null,
    shop varchar(250) not null,
    timestamp timestamp not null
);

create index ix_favoriteShops_user on favoriteShops (
    userid
);
