CREATE TABLE "order" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"datetime"	TEXT NOT NULL,
	"address"	TEXT NOT NULL,
	"phone"	TEXT NOT NULL,
	"status"	TEXT NOT NULL DEFAULT 'Draft'
)