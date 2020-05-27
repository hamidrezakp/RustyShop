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

$('.card').hover(function(){
  $(this).find(' > a.stay-right').addClass('stay-right-visible');
},function(){
  $(this).find(' > a.stay-right').removeClass('stay-right-visible');
})

function update_cart_count(cart) {
	let sum = cart.products.reduce((psum, item) => psum + item.quantity, 0);
	$('.buy-cart').children()[1].textContent = sum;
}

//
// Minicart
//
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
