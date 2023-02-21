class CartComp extends HTMLElement {

    constructor() {
        super();

        this.addEventListener('cartLoaded', e => {
            this.innerHTML = this.getCartDoc(e.detail);
        });
    }

    getCartDoc(detail) {
        return `<ol class="list-group">${this.getLineItems(detail)}</ol>`;
    }

    getLineItems(detail) {
        let lineItems = '';
        for (let s of detail) {
            lineItems += `<li class="list-group-item"><img src="${s}.jpg" alt="${s}" height="20" width="20">${s}</li>`;
        }
        return lineItems;
    }
}
window.customElements.define('cart-comp', CartComp);