import jQuery from 'jquery';

declare global {
    interface Window { // ⚠️ notice that "Window" is capitalized here
        $: any;
        jQuery: any;
    }
}

window.$ = window.jQuery = jQuery;

class CartComponent extends HTMLElement {

    constructor() {
        super();
        console.log("cart app constructor called");
    }

    createCartApp() {
        const root = $("#root");
        console.log("createCartApp");
        root.append("CartComponent JQUERY hello from jquery slkfjslj lsjf jsldjf ljsfl");
        return `<p>bli</p><p>bla</p><p>blupp</p><div><input id="searchtext2"  type="search" class="form-control" placeholder="Search..." aria-label="Search"/></div>`;
    }

    connectedCallback() {
        console.log("car app callback");
        this.innerHTML += this.createCartApp();
    }
}

window.customElements.define('cart-component', CartComponent);

console.log("halllo cart-component")

export let CartApp: () => string = function () {
    return `cart app  !`;
};

export default CartApp;
