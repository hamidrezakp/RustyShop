CREATE TABLE "ordered_products" (
	"order_id"	INTEGER NOT NULL,
	"product_id"	INTEGER NOT NULL,
	FOREIGN KEY("product_id") REFERENCES "product"("id") ON UPDATE CASCADE,
	FOREIGN KEY("order_id") REFERENCES "order"("id") ON UPDATE CASCADE,
	PRIMARY KEY("order_id","product_id")
)
