// Initial Cart update
if (localStorage && localStorage.getItem('cart')) {
	if (!localStorage){
		console.log("Browser is not supporting LocalStorage");
	} else {
		var cart = JSON.parse(localStorage.getItem('cart'));            

		update_cart_count(cart.products);
		load_minicart(cart.products);
	}
}

function addToCart(product) {
    // Retrieve the cart object from local storage
	var cart = JSON.parse(localStorage.getItem('cart'));            
    if (cart) {
		let saved_product = cart.products.find(e => e.id == product.id)
		if (saved_product != null) {
			saved_product.quantity += product.quantity;
			update_minicart(saved_product.id, saved_product.quantity);
		} else {
			cart.products.push(product);
			add_to_minicart(product);
		}

        localStorage.setItem('cart', JSON.stringify(cart));
    } else {
		var cart = {};
		cart.products = [product];
		localStorage.setItem('cart', JSON.stringify(cart));
		add_to_minicart(product);
	}
	update_cart_count(cart.products);
}

$('.addToCartBtn').on('click', function(e) {
	// TODO: Check quantity
	e.preventDefault();
	
    var product = {};
    product.id = $(this).attr('data-id');
    product.name = $(this).attr('data-name');
    product.price = $(this).attr('data-price');
    product.image = $(this).attr('data-image');
	product.quantity = 1;

    addToCart(product);
});

$('.card').hover(function(){
  $(this).find(' > a.stay-right').addClass('stay-right-visible');
},function(){
  $(this).find(' > a.stay-right').removeClass('stay-right-visible');
})

function update_cart_count(products) {
	let sum = products.reduce((psum, item) => psum + item.quantity, 0);
	$('.buy-cart').children('span')[0].textContent = sum;
	$('.minicart-header').children('span')[0].textContent = sum;
}

//
// Minicart
//
function load_minicart(products){
	products.forEach(add_to_minicart);	
	update_minicart_sum(products);
}

function open_minicart() {
	$('.minicart').addClass('minicart-open');
	$('.minicart-overlay').addClass('minicart-overlay-fadein');
	$('.minicart-inner').addClass('minicart-inner-open');
}

function close_minicart() {
	$('.minicart').removeClass('minicart-open');
	$('.minicart-overlay').removeClass('minicart-overlay-fadein');
	$('.minicart-inner').removeClass('minicart-inner-open');
}

$('.minicart-header').children('a').click(close_minicart);
$('.buy-cart').click(open_minicart);
$('.minicart-overlay').click(close_minicart);

function add_to_minicart(product) {
	var new_product_node = `
	<div class="minicart-product-row bg-light" data-id="${product.id}">
		<div class="minicart-product-image">
			<img src="/assets/img/product/${product.image}" alt="${product.name}" loading="lazy">
		</div>
		<div class="minicart-product-info">
			<div class="minicart-product-info-title">
				<h5>${product.name}</h5>
				<i class="fa fa-trash minicart-product-info-delIcon" aria-hidden="true"></i>
			</div>
			<div class="minicart-product-info-price">
				<div class="minicart-product-info-price-p text-success">$${product.price}</div>
				<div class="minicart-product-info-quantity">X ${product.quantity}</div>
			</div>
		</div>
	</div>`;
	$('.minicart-products').prepend(new_product_node);
	update_minicart_sum();
}

function update_minicart(product_id, new_quantity){
	var product = $('.minicart-products')
		.find(`.minicart-product-row[data-id=${product_id}]`)
		.find('.minicart-product-info-quantity')[0]
		.textContent = `X ${new_quantity}`;
	update_minicart_sum();
}

function update_minicart_sum() {
	var cart = JSON.parse(localStorage.getItem('cart'));            
	let sum = cart.products.reduce((psum, item) => psum + item.quantity * item.price, 0);
	$('.minicart-footer-total-price')[0].textContent = `$${sum}`;
}
