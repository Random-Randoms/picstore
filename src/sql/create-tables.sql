create table if not exists Usr (
	id bigint generated always as identity unique not null primary key,
	name varchar(64) unique not null
);

create table if not exists Scope (
	id bigint generated always as identity unique not null primary key,
	name varchar(64) unique not null
);

create table if not exists UsrScope (
	usr bigint not null,
	scope bigint not null,
	primary key (usr, scope),
	foreign key (usr) references Usr(id),
	foreign key (scope) references Scope(id)
);

create table if not exists Picture (
	id bigint generated always as identity unique not null primary key,
	scope bigint,
	foreign key (scope) references Scope(id)
)
