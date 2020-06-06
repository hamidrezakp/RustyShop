CREATE TABLE "Ordered_products" (
	"order_id"	INTEGER NOT NULL,
	"product_id"	INTEGER NOT NULL,
	"quantity"	INTEGER NOT NULL,
	FOREIGN KEY("product_id") REFERENCES "Products"("id") ON UPDATE CASCADE,
	FOREIGN KEY("order_id") REFERENCES "Orders"("id") ON UPDATE CASCADE,
	PRIMARY KEY("order_id","product_id")
)
