"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const priceservice_1 = require("./generated-clients/priceservice");
const price = () => {
    const configParams = {
        basePath: 'http://localhost:58800',
    };
    const apiConfig = new priceservice_1.Configuration(configParams);
    const api = new priceservice_1.DefaultApi(apiConfig);
    const r = {
        email: "login_req.email"
    };
};
//# sourceMappingURL=services.js.map