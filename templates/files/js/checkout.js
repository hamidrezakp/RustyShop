// Initial products update
if (!localStorage){
    console.log("Browser is not supporting LocalStorage");
} else {
    var cart = JSON.parse(localStorage.getItem('cart'));            
    if (cart) {
        var cart = JSON.parse(localStorage.getItem('cart'));            
        load_products(cart.products);
				$('.minicart-product-info-delIcon').on('click', function(product) {
					remove_product(this);
				});
        update();
    } else {
        var cart = {};
        cart.products = [];
        localStorage.setItem('cart', JSON.stringify(cart));
    }
}

function update_cart_count() {
	//TODO
	/*
	var cart = JSON.parse(localStorage.getItem('cart'));            
	let sum = cart.products.reduce((psum, item) => psum + item.quantity, 0);
	$('.buy-cart').children('span')[0].textContent = sum;
	$('.minicart-header').children('span')[0].textContent = sum;
	*/
}

function update() {
	update_products_sum();
	update_cart_count();
}

function load_products(products){
	products.forEach(add_to_page);	
}

function update_products_sum() {
	// TODO
	/*
	var cart = JSON.parse(localStorage.getItem('cart'));            
	let sum = cart.products.reduce((psum, item) => psum + item.quantity * item.price, 0);
	$('.')[0].textContent = `$${sum}`;
	*/
}

function add_to_page(product) {
	var new_product_node = `
				<div class="checkout-products-table-group mt-4" role="row" data-id="${product.id}">
					<div class="checkout-products-table-details checkout-products-table-cell" role="gridcell">
							<div class="checkout-products-table-image-wrapper">
								<img class="checkout-products-table-image" src="/assets/img/product/${product.image}" alt="">
							</div>
							<div class="checkout-products-table-text">
								<h6 class="">${product.name}</h6>
							</div>
					</div>
					<div class="checkout-products-table-100w checkout-products-table-cell" role="gridcell">
						$${product.price}
					</div>
					<div class="checkout-products-table-100w checkout-products-table-cell" role="gridcell">
						${product.quantity}
					</div>
					<div class="checkout-products-table-100w checkout-products-table-cell" role="gridcell">
						$${product.quantity * product.price}
					</div>
					<div class="checkout-products-table-cell" role="gridcell" style="flex: 40 0 auto; width: 40px; max-width: 40px;">
						<i aria-hidden="true" class="fa fa-trash h4 minicart-product-info-delIcon" data-id="${product.id}"></i>
					</div>
				</div>`;
	$('.checkout-products-table-body').prepend(new_product_node);
	update();
}

function remove_product(node){
	let node_id = $(node).attr('data-id');
	var cart = JSON.parse(localStorage.getItem('cart'));            

	cart.products = cart.products.filter(function( item ) {
	    return item.id !== node_id;
	});

        localStorage.setItem('cart', JSON.stringify(cart));

	$(`.checkout-products-table-group[data-id=${node_id}]`).remove();

	update();
}

