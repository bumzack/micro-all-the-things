import {Configuration, ConfigurationParameters, DefaultApi} from "./generated-clients/priceservice";
import {GetCustomerRequest} from "./generated-clients/customerservice";

const price = (): void => {
    const configParams: ConfigurationParameters = {
        basePath: 'http://localhost:58800',
    };

    const apiConfig = new Configuration(configParams);
    const api = new DefaultApi(apiConfig);
    const r: GetCustomerRequest = {
        email: "login_req.email"
    };
}
