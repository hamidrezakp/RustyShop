CREATE TABLE "payment" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"datetime"	TEXT NOT NULL,
	"amount"	REAL NOT NULL DEFAULT 0,
	"order_id"	INTEGER NOT NULL,
	"customer_id"	INTEGER NOT NULL,
	FOREIGN KEY("customer_id") REFERENCES "user"("id") ON UPDATE CASCADE,
	FOREIGN KEY("order_id") REFERENCES "order"("id") ON UPDATE CASCADE
)