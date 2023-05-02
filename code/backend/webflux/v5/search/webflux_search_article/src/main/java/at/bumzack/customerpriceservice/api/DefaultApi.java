package at.bumzack.customerpriceservice.api;

import at.bumzack.customerpriceservice.ApiClient;
import at.bumzack.customerpriceservice.model.CustomerPriceEntry;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.ParameterizedTypeReference;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpMethod;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.util.LinkedMultiValueMap;
import org.springframework.util.MultiValueMap;
import org.springframework.web.reactive.function.client.WebClient.ResponseSpec;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2023-04-30T02:11:33.492992+02:00[Europe/Vienna]")
public class DefaultApi {
    private ApiClient apiClient;

    public DefaultApi() {
        this(new ApiClient());
    }

    @Autowired
    public DefaultApi(ApiClient apiClient) {
        this.apiClient = apiClient;
    }

    public ApiClient getApiClient() {
        return apiClient;
    }

    public void setApiClient(ApiClient apiClient) {
        this.apiClient = apiClient;
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param customerId Customer ID
     * @return List&lt;CustomerPriceEntry&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    private ResponseSpec customerPricesRequestCreation(Long customerId) throws WebClientResponseException {
        Object postBody = null;
        // verify the required parameter 'customerId' is set
        if (customerId == null) {
            throw new WebClientResponseException("Missing the required parameter 'customerId' when calling customerPrices", HttpStatus.BAD_REQUEST.value(), HttpStatus.BAD_REQUEST.getReasonPhrase(), null, null, null);
        }
        // create path and map variables
        final Map<String, Object> pathParams = new HashMap<String, Object>();

        pathParams.put("customerId", customerId);

        final MultiValueMap<String, String> queryParams = new LinkedMultiValueMap<String, String>();
        final HttpHeaders headerParams = new HttpHeaders();
        final MultiValueMap<String, String> cookieParams = new LinkedMultiValueMap<String, String>();
        final MultiValueMap<String, Object> formParams = new LinkedMultiValueMap<String, Object>();

        final String[] localVarAccepts = {
                "*/*"
        };
        final List<MediaType> localVarAccept = apiClient.selectHeaderAccept(localVarAccepts);
        final String[] localVarContentTypes = {};
        final MediaType localVarContentType = apiClient.selectHeaderContentType(localVarContentTypes);

        String[] localVarAuthNames = new String[]{};

        ParameterizedTypeReference<CustomerPriceEntry> localVarReturnType = new ParameterizedTypeReference<CustomerPriceEntry>() {
        };
        return apiClient.invokeAPI("/api/v1/customerprices/{customerId}", HttpMethod.GET, pathParams, queryParams, postBody, headerParams, cookieParams, formParams, localVarAccept, localVarContentType, localVarAuthNames, localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param customerId Customer ID
     * @return List&lt;CustomerPriceEntry&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Flux<CustomerPriceEntry> customerPrices(Long customerId) throws WebClientResponseException {
        ParameterizedTypeReference<CustomerPriceEntry> localVarReturnType = new ParameterizedTypeReference<CustomerPriceEntry>() {
        };
        return customerPricesRequestCreation(customerId).bodyToFlux(localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param customerId Customer ID
     * @return ResponseEntity&lt;List&lt;CustomerPriceEntry&gt;&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<ResponseEntity<List<CustomerPriceEntry>>> customerPricesWithHttpInfo(Long customerId) throws WebClientResponseException {
        ParameterizedTypeReference<CustomerPriceEntry> localVarReturnType = new ParameterizedTypeReference<CustomerPriceEntry>() {
        };
        return customerPricesRequestCreation(customerId).toEntityList(localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param customerId Customer ID
     * @return ResponseSpec
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public ResponseSpec customerPricesWithResponseSpec(Long customerId) throws WebClientResponseException {
        return customerPricesRequestCreation(customerId);
    }
}
