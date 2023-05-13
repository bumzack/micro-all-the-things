import {
    AuthenticationEntry,
    Configuration,
    ConfigurationParameters,
    DefaultApi,
    LoggedinRequest
} from "./generated-clients/authenticationservice";


export const is_authenticated = async (customer_id: number): Promise<AuthenticationEntry | undefined> => {
    const configParams: ConfigurationParameters = {
        basePath: 'http://localhost:58982',
    };

    const apiConfig = new Configuration(configParams);
    const api = new DefaultApi(apiConfig);
    let req: LoggedinRequest = {
        customerId: customer_id,
    }
    console.log(`authentication request    ${JSON.stringify(req, null, 4)}`);
    return await api.loggedin(req)
        .then(r => {
            return r;
        })
        .catch(err => {
            console.log("error requesting authentication ", err);
            return undefined;
        });
}