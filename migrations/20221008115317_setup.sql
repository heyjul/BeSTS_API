-- Add migration script here
CREATE TABLE user (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   	email TEXT NOT NULL,
	password TEXT NOT NULL,
	username TEXT NOT NULL,
	UNIQUE (email) 
);

CREATE TABLE room (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   	name TEXT NOT NULL,
	owner_id INTEGER NOT NULL,
	FOREIGN KEY (owner_id) REFERENCES user(id)
);

CREATE TABLE room_user (
	room_id INTEGER NOT NULL,
	user_id INTEGER NOT NULL,
    PRIMARY KEY (room_id, user_id),
   	FOREIGN KEY (room_id) REFERENCES room(id),
   	FOREIGN KEY (user_id) REFERENCES user(id)
);

CREATE TABLE team (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL
);

CREATE TABLE match (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	start_date DATETIME NOT NULL,
	winner_points INTEGER NOT NULL,
	guess_points INTEGER NOT NULL,
	team_one_id INTEGER NOT NULL,
	team_two_id INTEGER NOT NULL,
	room_id INTEGER NOT NULL,
	FOREIGN KEY (team_one_id) REFERENCES team(id),
	FOREIGN KEY (team_two_id) REFERENCES team(id),
   	FOREIGN KEY (room_id) REFERENCES room(id),
	CHECK (team_one_id != team_two_id),
	CHECK (guess_points >= winner_points)
);

CREATE TABLE bet (
	match_id INTEGER NOT NULL,
	user_id INTEGER NOT NULL,
	team_one_score INTEGER NOT NULL,
	team_two_score INTEGER NOT NULL,
	PRIMARY KEY (match_id, user_id),
	FOREIGN KEY (match_id) REFERENCES match(id),
	FOREIGN KEY (user_id) REFERENCES user(id)
);

CREATE TABLE result (
	match_id INTEGER NOT NULL PRIMARY KEY,
	team_one_score INTEGER NOT NULL,
	team_two_score INTEGER NOT NULL,
	FOREIGN KEY (match_id) REFERENCES match(id)
);