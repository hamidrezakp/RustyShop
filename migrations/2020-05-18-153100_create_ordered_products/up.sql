CREATE TABLE "ordered_products" (
	"order"	INTEGER NOT NULL,
	"product"	INTEGER NOT NULL,
	FOREIGN KEY("product") REFERENCES "Product"("id") ON UPDATE CASCADE,
	FOREIGN KEY("order") REFERENCES "Order"("id") ON UPDATE CASCADE,
	PRIMARY KEY("order","product")
)
