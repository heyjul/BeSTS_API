-- Add migration script here
CREATE TABLE users (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   	email TEXT NOT NULL,
	password TEXT NOT NULL,
	username TEXT NOT NULL,
	UNIQUE (email) 
);

CREATE TABLE rooms (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   	name TEXT NOT NULL,
	owner INTEGER NOT NULL,
	FOREIGN KEY (owner) REFERENCES user(id)
);

CREATE TABLE rooms_users (
	room_id INTEGER NOT NULL,
	user_id INTEGER NOT NULL,
    PRIMARY KEY (room_id, user_id),
   	FOREIGN KEY (room_id) REFERENCES room(id),
   	FOREIGN KEY (user_id) REFERENCES user(id)
);

CREATE TABLE matches (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	closed_date TEXT NOT NULL,
	room_id INTEGER NOT NULL,
   	FOREIGN KEY (room_id) REFERENCES room(id)
);

CREATE TABLE teams (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	match_id INTEGER NOT NULL,
	player_one INTEGER NOT NULL CHECK(player_one in (0, 1)),
   	FOREIGN KEY (match_id) REFERENCES match(id)
	UNIQUE (match_id, player_one)
);

CREATE TABLE bets (
	team_id INTEGER NOT NULL,
	user_id INTEGER NOT NULL,
	score INTEGER NOT NULL,
    PRIMARY KEY (team_id, user_id),
   	FOREIGN KEY (team_id) REFERENCES team(id),
   	FOREIGN KEY (user_id) REFERENCES user(id) 
);