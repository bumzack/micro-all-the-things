"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.is_authenticated = void 0;
const authenticationservice_1 = require("./generated-clients/authenticationservice");
const is_authenticated = async (customer_id) => {
    const configParams = {
        basePath: 'http://localhost:58982',
    };
    const apiConfig = new authenticationservice_1.Configuration(configParams);
    const api = new authenticationservice_1.DefaultApi(apiConfig);
    let req = {
        customerId: customer_id,
    };
    return await api.loggedin(req);
};
exports.is_authenticated = is_authenticated;
//# sourceMappingURL=services_auth.js.map