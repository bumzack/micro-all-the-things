import {
    Configuration,
    ConfigurationParameters,
    CustomerPriceEntry,
    CustomerPricesRequest,
    DefaultApi
} from "./generated-clients/customerpriceservice";

export const find_customer_prices = async (id: number): Promise<Array<CustomerPriceEntry>> => {
    const configParams: ConfigurationParameters = {
        basePath: 'http://localhost:58981',
    };

    const apiConfig = new Configuration(configParams);
    const api = new DefaultApi(apiConfig);

    let req: CustomerPricesRequest = {
        customerId: id
    };
    console.log(`find_customer_prices   request ${JSON.stringify(req)} `);
    return await api.customerPrices(req);
}