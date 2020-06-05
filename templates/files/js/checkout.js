// Constants
//
const POSTCOST = 12
//
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

init_nav_btns();
change_section('products');
set_click_handler_for_finish();


function set_click_handler_for_finish() {
	$('.checkout-navigations-btn[data-target=finish]')
		.on('click', function(e) {
			e.preventDefault();
			submit_form();
	});
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
	var cart = JSON.parse(localStorage.getItem('cart'));            
	let sum = cart.products.reduce((psum, item) => psum + item.quantity * item.price, 0);
	$('.checkout-products-costs-value').text(`$${sum}`);
	$('.checkout-post-cost').text(`$${POSTCOST}`);
	$('.checkout-total-cost').text(`$${sum + POSTCOST}`);
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

function change_section(new_section) {
	let sections = ['products', 'sentdetails', 'payment', 'finish'];

	if (new_section == 'shop') {
		window.location = '/';
		return;
	}

	sections.forEach(function(section) {
		if (section == new_section) {
			$(`.checkout-${section}`).addClass('checkout-active');
			$(`.pipeline-${section}`).addClass('pipeline-step-active');
			$(`.pipeline-${section}`).find('.pipeline-line')
				.addClass('pipeline-ln-active');
		} else {
			$(`.checkout-${section}`).removeClass('checkout-active');
			$(`.pipeline-${section}`).removeClass('pipeline-step-active');
			$(`.pipeline-${section}`).find('.pipeline-line')
				.removeClass('pipeline-ln-active');
		}
	});	
}

function init_nav_btns() {
	$('.checkout-navigations-btn').on('click', function(e) {
		e.preventDefault();
		change_section($(this).attr('data-target'));
	});
}

function submit_form() {
	let form = $('#checkout-form');
	let url = form.attr('action');

	var products_quantities = JSON.parse(localStorage.getItem('cart')).products.map(
		function(item) { return [parseInt(item.id), parseFloat(item.quantity)]});            


	let data = {
		// TODO change userid
		user_id: 1,
		products: products_quantities,
		address: form.find('input[name=address]')[0].value,
		phonenumber: form.find('input[name=phonenumber]')[0].value,
	};

	$.ajax({
		url: url,
		type: "POST",
		data: JSON.stringify(data),
		dataType: 'json',
		async: true,
		contentType: 'application/json; charset=utf-8',
	});
		localStorage.setItem('cart', JSON.stringify({products:[]}));
}
