console.log('Hello world!');
const p = {
    code: "code",
    name: "hallo"
};
console.log(`product  ${JSON.stringify(p, null, 4)}`);
class ProductsComp extends HTMLElement {
    constructor() {
        super();
        console.log("construtor Productscomp");
        this.addEventListener('change', e => {
            // @ts-ignore
            if (e.target.type === "checkbox") {
                let atleastOneChecked = false;
                for (let el of this.getElementsByTagName('input')) {
                    if (el.checked) {
                        atleastOneChecked = true;
                        break;
                    }
                }
                //document.getElementById("productsCompBtn").disabled = !atleastOneChecked;
            }
        });
        console.info("asasas");
        this.addEventListener('click', e => {
            console.info("addEventListener");
            // @ts-ignore
            if (e.target.type === "submit") {
                let selectedFruits = new Array();
                for (let el of this.getElementsByTagName('input')) {
                    if (el.checked) {
                        selectedFruits.push(el.getAttribute("value"));
                    }
                }
                if (selectedFruits.length >= 0) {
                    this.dispatchEvent(new CustomEvent('selectedFruits', {
                        detail: selectedFruits
                    }));
                }
            }
        });
    }
    connectedCallback() {
        this.innerHTML += this.createProductsApp();
    }
    createProductsApp() {
        return `<div>
        <div class="row row-cols-3">
            <div class="col-sm"><img src="img/apple.jpg" height="150" width="150"><br />
            <span style="color: black;">Apple $1.00/lb</span> 
            <input type="checkbox" name="fruits" value="apple">
            </div>
            <div class="col-sm"><img src="img/kiwi.jpg" height="150" width="150"><br />
            <span style="color: black;">Kiwi $2.00/lb</span> <input type="checkbox" name="fruits" value="kiwi">
            </div>
            <div class="col-sm"><img src="img/orange.jpg" height="150" width="150"><br />
            <span style="color: black;">Orange $1.00/lb</span> <input type="checkbox" name="fruits" value="orange">
            </div>
            </div>
        </div>
        <div class="row row row-cols-3"><div class="col-sm">&nbsp;</div></div>
        <div class="row row-cols-3"><div class="col-sm d-flex justify-content-center">
        <p><button id="productsCompBtn" class="btn btn-primary">Add to Cart</button></p></div></div>`;
    }
}
window.customElements.define('products-comp', ProductsComp);
export {};
