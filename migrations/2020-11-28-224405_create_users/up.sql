CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users
(
	id uuid not null default uuid_generate_v1(),
	email varchar not null,
	password_hash varchar not null,
	password_salt varchar not null,
	fail_logins int default 0,
	locked_until timestamptz,
	created_at timestamptz not null default current_timestamp
);

comment on column users.fail_logins is 'Number of fail login attempts';

comment on column users.locked_until is 'User is locked. The timestamp represent the next ealiest time the user can login';

create unique index users_email_uindex
	on users (email);

create unique index users_id_uindex
	on users (id);

alter table users
	add constraint users_pk
		primary key (id);


