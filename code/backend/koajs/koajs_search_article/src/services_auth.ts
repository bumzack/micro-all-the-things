import {
    AuthenticationEntry,
    Configuration,
    ConfigurationParameters,
    DefaultApi,
    LoggedinRequest
} from "./generated-clients/authenticationservice";


export const is_authenticated = async (customer_id: number): Promise<AuthenticationEntry> => {
    const configParams: ConfigurationParameters = {
        basePath: 'http://localhost:58982',
    };

    const apiConfig = new Configuration(configParams);
    const api = new DefaultApi(apiConfig);
    let req: LoggedinRequest = {
        customerId: customer_id,
    }
    return await api.loggedin(req);
}