"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.find_customer_prices = void 0;
const customerpriceservice_1 = require("./generated-clients/customerpriceservice");
const find_customer_prices = async (id) => {
    const configParams = {
        basePath: 'http://localhost:58981',
    };
    const apiConfig = new customerpriceservice_1.Configuration(configParams);
    const api = new customerpriceservice_1.DefaultApi(apiConfig);
    let req = {
        customerId: id
    };
    console.log(`find_customer_prices   request ${JSON.stringify(req)} `);
    return await api.customerPrices(req);
};
exports.find_customer_prices = find_customer_prices;
//# sourceMappingURL=services_customerprices.js.map