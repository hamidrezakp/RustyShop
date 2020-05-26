// Initial Cart object in browser storage
var cart = {};
cart.products = [];

localStorage.setItem('cart', JSON.stringify(cart));


function addToCart(product) {
    // Retrieve the cart object from local storage
    if (localStorage && localStorage.getItem('cart')) {
        var cart = JSON.parse(localStorage.getItem('cart'));            
			
		let savedProduct = cart.products.find(e => e.id == product.id)
		if (savedProduct != null) {
			savedProduct.quantity += product.quantity;
		} else {
			cart.products.push(product);
		}

        localStorage.setItem('cart', JSON.stringify(cart));
		update_cart_count(cart);
    } 
}

$('.addToCartBtn').on('click', function(e) {
	// TODO: Check quantity
	e.preventDefault();
	
    var product = {};
    product.id = $(this).attr('data-id');
    product.name = $(this).attr('data-name');
    product.price = $(this).attr('data-price');
	product.quantity = 1;

    addToCart(product);
});

function update_cart_count(cart) {
	let sum = cart.products.reduce((psum, item) => psum + item.quantity, 0);
	$('.buy-cart').children()[1].textContent = sum;
}
