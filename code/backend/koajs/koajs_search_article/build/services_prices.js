"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.find_prices = void 0;
const priceservice_1 = require("./generated-clients/priceservice");
const find_prices = async (tconsts) => {
    const configParams = {
        basePath: 'http://localhost:58800',
    };
    const apiConfig = new priceservice_1.Configuration(configParams);
    const api = new priceservice_1.DefaultApi(apiConfig);
    let searchPricesRequest = {
        movieTconst: tconsts
    };
    let req = {
        searchPricesRequest: searchPricesRequest
    };
    return await api.getPricesForMovies(req);
};
exports.find_prices = find_prices;
//# sourceMappingURL=services_prices.js.map