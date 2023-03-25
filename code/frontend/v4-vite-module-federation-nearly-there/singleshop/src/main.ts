async function init() {
    console.log("single shop callback");
    let shop = await import("shop/ShopApp");
    const a = shop.default();
   console.log("shop ", a);
}

console.log("before init singleshop");
init()
console.log("after init singleshop");
