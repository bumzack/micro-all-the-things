class ForYouAndYourFakeShop extends HTMLElement {
    constructor() {
        super();
        console.log("construtor ForYouAndYourFakeShop");
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
        console.info("asasas")

        this.addEventListener('click', e => {
            console.info("addEventListener")
            // @ts-ignore
            if (e.target.type === "submit") {
                let selectedFruits = new Array();
                for (let el of this.getElementsByTagName('input')) {
                    if (el.checked) {
                        selectedFruits.push(el.getAttribute("value"));
                    }
                }

                if (selectedFruits.length >= 0) {
                    this.dispatchEvent(
                        new CustomEvent('selectedFruits', {
                            detail: selectedFruits
                        })
                    );
                }
            }
        });

    }

    connectedCallback() {

        this.innerHTML += this.createProductsApp();
    }

    createProductsApp() {
        return `
<header class="bd-header bg-dark  py-3 d-flex align-items-stretch border-bottom border-dark">
    <div class="container-fluid d-grid  gap-3 align-items-center" style="grid-template-columns: 1fr 2fr;">
        <div class="dropdown">
            <a href="#" class="d-flex align-items-center col-lg-4 mb-2 mb-lg-0   text-decoration-none dropdown-toggle" data-bs-toggle="dropdown" aria-expanded="false">
                <svg class="bi me-2" width="40" height="32">
                    <use xlink:href="#bootstrap">
                        <symbol id="bootstrap" viewBox="0 0 118 94">
                            <title>Bootstrap</title>
                            <path fill-rule="evenodd" clip-rule="evenodd"
                                  d="M24.509 0c-6.733 0-11.715 5.893-11.492 12.284.214 6.14-.064 14.092-2.066 20.577C8.943 39.365 5.547 43.485 0 44.014v5.972c5.547.529 8.943 4.649 10.951 11.153 2.002 6.485 2.28 14.437 2.066 20.577C12.794 88.106 17.776 94 24.51 94H93.5c6.733 0 11.714-5.893 11.491-12.284-.214-6.14.064-14.092 2.066-20.577 2.009-6.504 5.396-10.624 10.943-11.153v-5.972c-5.547-.529-8.934-4.649-10.943-11.153-2.002-6.484-2.28-14.437-2.066-20.577C105.214 5.894 100.233 0 93.5 0H24.508zM80 57.863C80 66.663 73.436 72 62.543 72H44a2 2 0 01-2-2V24a2 2 0 012-2h18.437c9.083 0 15.044 4.92 15.044 12.474 0 5.302-4.01 10.049-9.119 10.88v.277C75.317 46.394 80 51.21 80 57.863zM60.521 28.34H49.948v14.934h8.905c6.884 0 10.68-2.772 10.68-7.727 0-4.643-3.264-7.207-9.012-7.207zM49.948 49.2v16.458H60.91c7.167 0 10.964-2.876 10.964-8.281 0-5.406-3.903-8.178-11.425-8.178H49.948z"></path>
                        </symbol>
                    </use>
                </svg>
            </a>
            <ul class="dropdown-menu text-small shadow">
                <li><a class="dropdown-item active" href="#" aria-current="page">Overview</a></li>
                <li><a class="dropdown-item" href="#">Inventory</a></li>
                <li><a class="dropdown-item" href="#">Customers</a></li>
                <li><a class="dropdown-item" href="#">Products</a></li>
                <li>
                    <hr class="dropdown-divider">
                </li>
                <li><a class="dropdown-item" href="#">Reports</a></li>
                <li><a class="dropdown-item" href="#">Analytics</a></li>
            </ul>
        </div>

        <div class="d-flex align-items-center">
            <div class="w-100 me-3" role="search">
                <search-comp id="searchapp"></search-comp>
            </div>

            <div class="flex-shrink-0 dropdown">
                <a href="#" class="d-block link-dark text-decoration-none dropdown-toggle" data-bs-toggle="dropdown" aria-expanded="false">
                    <img src="https://github.com/mdo.png" alt="mdo" class="rounded-circle" width="32" height="32">
                </a>
                <ul class="dropdown-menu text-small shadow">
                    <li><a class="dropdown-item" href="#">New project...</a></li>
                    <li><a class="dropdown-item" href="#">Settings</a></li>
                    <li><a class="dropdown-item" href="#">Profile</a></li>
                    <li>
                        <hr class="dropdown-divider">
                    </li>
                    <li><a class="dropdown-item" href="#">Sign out</a></li>
                </ul>
            </div>
        </div>
    </div>
</header>


<div class="bd-cheatsheet container-fluid bg-body">
    <section id="content">
        <h2 class="sticky-xl-top fw-bold pt-3 pt-xl-5 pb-2 pb-xl-3">Contents</h2>


        <article class="my-3" id="card">
            <category-comp id="categoryapp"></category-comp>

            <div>
                <div class="bd-example-snippet bd-code-snippet">
                    <div class="bd-example">
                        <div class="row  row-cols-1 row-cols-md-2 g-4">
                            <search-result-comp id="search-result-app"></search-result-comp>
                        </div>
                    </div>
                </div>
            </div>
        </article>


    </section>
</div>


<script src="http://localhost:3000/js/require.js"></script>


<script src="http://localhost:3000/dist/main.js"></script>
<script src="http://localhost:3000/dist/cheatsheet.js"></script>


<!-- web components -->
<!--<script type="module" src="http://www.bumzack.at:3011/dist/productsapp/src/products-comp.js"></script>-->
<script src="http://localhost:4011/index.0dde725b.js"></script>
<script src="http://www.bumzack.at:3012/dist/main.js"></script>
<script src="http://www.bumzack.at:3013/dist/main.js"></script>
<script src="http://www.bumzack.at:3014/dist/main.js"></script>

<!--<script type="module" src="http://localhost:3000/dist/common/common.js"></script>-->
<!--<script type="module" src="http://localhost:3000/dist/common/dto.js"></script>-->

 `;
    }
}

export {};

window.customElements.define('for-you-and-your-fake-shop-comp', ForYouAndYourFakeShop);
