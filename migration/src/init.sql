drop table if exists sys_user;
drop table if exists sys_role;
drop table if exists sys_menu;
drop table if exists sys_user_role;
drop table if exists sys_role_menu;
drop table if exists sys_api;
drop table if exists sys_dictionary;
drop table if exists sys_dictionary_detail;

create table sys_user
(
    id              integer primary key autoincrement,
    username        text,
    password        text,
    nick_name       text      default '系统用户',
    side_mode       text      default 'dark',
    header_img      blob,
    base_color      text      default '#fff',
    active_color    text      default '#1890ff',
    phone           text,
    email           text,
    has_super_admin boolean   default true,
    status          boolean   default true,
    created_at      timestamp default current_timestamp,
    updated_at      timestamp default current_timestamp
);

create table sys_role
(
    id            integer primary key autoincrement,
    name          text,
    default_route text,
    status        boolean   default true,
    created_at    timestamp default current_timestamp,
    updated_at    timestamp default current_timestamp
);

create table sys_menu
(
    id         integer primary key autoincrement,
    parent_id  integer,
    title      text,
    icon       text,
    name       text,
    path       text,
    component  text,
    hidden     boolean   default true,
    sort       integer,
    status     boolean   default true,
    created_at timestamp default current_timestamp,
    updated_at timestamp default current_timestamp
);

create table sys_user_role
(
    user_id integer
        constraint fk_sys_user_role_sys_user references sys_user,
    role_id integer
        constraint fk_sys_user_role_sys_role references sys_role,
    primary key (user_id, role_id)
);

create table sys_role_menu
(
    role_id integer
        constraint fk_sys_role_menu_sys_role references sys_role,
    menu_id integer
        constraint fk_sys_role_menu_sys_base_menu references sys_menu,
    primary key (role_id, menu_id)
);

create table sys_api
(
    id          integer primary key autoincrement,
    path        text,
    description text,
    api_group   text,
    method      text      default 'POST',
    status      boolean   default true,
    created_at  timestamp default current_timestamp,
    updated_at  timestamp default current_timestamp
);

create table sys_dictionary
(
    id         integer primary key autoincrement,
    name       text,
    alias      text,
    desc       text,
    status     boolean   default true,
    created_at timestamp default current_timestamp,
    updated_at timestamp default current_timestamp
);

create table sys_dictionary_detail
(
    id            integer primary key autoincrement,
    dictionary_id integer
        constraint fk_sys_dictionary_dictionary_details references sys_dictionary,
    label         text,
    value         integer,
    sort          integer,
    status        boolean   default true,
    created_at    timestamp default current_timestamp,
    updated_at    timestamp default current_timestamp
);