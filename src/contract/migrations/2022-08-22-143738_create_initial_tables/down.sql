-- This file should undo anything in `up.sql`
drop index ix_favoriteShops_user;
drop table favoriteShops;

drop index ix_favoriteProducts_user;
drop table favoriteProducts;

drop index ix_wishlist_user;
drop table wishlist;

drop index ix_purchases_user;
drop table purchases;