create table if not exists files (
  id         integer    primary key autoincrement,
  name       text       not null,
  path       text       not null,
  size       integer    not null,
  contents   blob       not null,
  created_at timestamp  default current_timestamp,
  updated_at timestamp  default current_timestamp
);

create unique index if not exists files_name_path on files (name, path);
