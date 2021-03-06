create table session_keys
(
	id uuid not null default uuid_generate_v1(),
	userId uuid not null
		constraint session_keys_users_id_fk
			references users
				on delete cascade,
	key varchar not null,
	expiry timestamptz not null,
	extended_at timestamptz,
	created_at timestamptz not null default current_timestamp
);

comment on column session_keys.expiry is 'When the session key will expire';

comment on column session_keys.extended_at is 'Last time the session key is extended';

create unique index session_keys_id_uindex
	on session_keys (id);

alter table session_keys
	add constraint session_keys_pk
		primary key (id);

