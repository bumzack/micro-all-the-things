package at.bumzack.authenticationservice.api;

import at.bumzack.authenticationservice.ApiClient;
import at.bumzack.authenticationservice.model.AuthenticationEntry;
import at.bumzack.authenticationservice.model.LogInRequest;
import at.bumzack.authenticationservice.model.LogOutRequest;
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
import reactor.core.publisher.Mono;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2023-04-29T15:35:02.276511+02:00[Europe/Vienna]")
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
     * @return AuthenticationEntry
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    private ResponseSpec loggedinRequestCreation(Long customerId) throws WebClientResponseException {
        Object postBody = null;
        // verify the required parameter 'customerId' is set
        if (customerId == null) {
            throw new WebClientResponseException("Missing the required parameter 'customerId' when calling loggedin", HttpStatus.BAD_REQUEST.value(), HttpStatus.BAD_REQUEST.getReasonPhrase(), null, null, null);
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

        ParameterizedTypeReference<AuthenticationEntry> localVarReturnType = new ParameterizedTypeReference<AuthenticationEntry>() {
        };
        return apiClient.invokeAPI("/api/v1/authenticated/{customerId}", HttpMethod.GET, pathParams, queryParams, postBody, headerParams, cookieParams, formParams, localVarAccept, localVarContentType, localVarAuthNames, localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param customerId Customer ID
     * @return AuthenticationEntry
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<AuthenticationEntry> loggedin(Long customerId) throws WebClientResponseException {
        ParameterizedTypeReference<AuthenticationEntry> localVarReturnType = new ParameterizedTypeReference<AuthenticationEntry>() {
        };
        return loggedinRequestCreation(customerId).bodyToMono(localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param customerId Customer ID
     * @return ResponseEntity&lt;AuthenticationEntry&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<ResponseEntity<AuthenticationEntry>> loggedinWithHttpInfo(Long customerId) throws WebClientResponseException {
        ParameterizedTypeReference<AuthenticationEntry> localVarReturnType = new ParameterizedTypeReference<AuthenticationEntry>() {
        };
        return loggedinRequestCreation(customerId).toEntity(localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param customerId Customer ID
     * @return ResponseSpec
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public ResponseSpec loggedinWithResponseSpec(Long customerId) throws WebClientResponseException {
        return loggedinRequestCreation(customerId);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param logInRequest The logInRequest parameter
     * @return AuthenticationEntry
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    private ResponseSpec loginRequestCreation(LogInRequest logInRequest) throws WebClientResponseException {
        Object postBody = logInRequest;
        // create path and map variables
        final Map<String, Object> pathParams = new HashMap<String, Object>();

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

        ParameterizedTypeReference<AuthenticationEntry> localVarReturnType = new ParameterizedTypeReference<AuthenticationEntry>() {
        };
        return apiClient.invokeAPI("/api/v1/authentication/login", HttpMethod.POST, pathParams, queryParams, postBody, headerParams, cookieParams, formParams, localVarAccept, localVarContentType, localVarAuthNames, localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param logInRequest The logInRequest parameter
     * @return AuthenticationEntry
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<AuthenticationEntry> login(LogInRequest logInRequest) throws WebClientResponseException {
        ParameterizedTypeReference<AuthenticationEntry> localVarReturnType = new ParameterizedTypeReference<AuthenticationEntry>() {
        };
        return loginRequestCreation(logInRequest).bodyToMono(localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param logInRequest The logInRequest parameter
     * @return ResponseEntity&lt;AuthenticationEntry&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<ResponseEntity<AuthenticationEntry>> loginWithHttpInfo(LogInRequest logInRequest) throws WebClientResponseException {
        ParameterizedTypeReference<AuthenticationEntry> localVarReturnType = new ParameterizedTypeReference<AuthenticationEntry>() {
        };
        return loginRequestCreation(logInRequest).toEntity(localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param logInRequest The logInRequest parameter
     * @return ResponseSpec
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public ResponseSpec loginWithResponseSpec(LogInRequest logInRequest) throws WebClientResponseException {
        return loginRequestCreation(logInRequest);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param logOutRequest The logOutRequest parameter
     * @return AuthenticationEntry
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    private ResponseSpec logoutRequestCreation(LogOutRequest logOutRequest) throws WebClientResponseException {
        Object postBody = logOutRequest;
        // create path and map variables
        final Map<String, Object> pathParams = new HashMap<String, Object>();

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

        ParameterizedTypeReference<AuthenticationEntry> localVarReturnType = new ParameterizedTypeReference<AuthenticationEntry>() {
        };
        return apiClient.invokeAPI("/api/v1/authentication/logout", HttpMethod.POST, pathParams, queryParams, postBody, headerParams, cookieParams, formParams, localVarAccept, localVarContentType, localVarAuthNames, localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param logOutRequest The logOutRequest parameter
     * @return AuthenticationEntry
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<AuthenticationEntry> logout(LogOutRequest logOutRequest) throws WebClientResponseException {
        ParameterizedTypeReference<AuthenticationEntry> localVarReturnType = new ParameterizedTypeReference<AuthenticationEntry>() {
        };
        return logoutRequestCreation(logOutRequest).bodyToMono(localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param logOutRequest The logOutRequest parameter
     * @return ResponseEntity&lt;AuthenticationEntry&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<ResponseEntity<AuthenticationEntry>> logoutWithHttpInfo(LogOutRequest logOutRequest) throws WebClientResponseException {
        ParameterizedTypeReference<AuthenticationEntry> localVarReturnType = new ParameterizedTypeReference<AuthenticationEntry>() {
        };
        return logoutRequestCreation(logOutRequest).toEntity(localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param logOutRequest The logOutRequest parameter
     * @return ResponseSpec
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public ResponseSpec logoutWithResponseSpec(LogOutRequest logOutRequest) throws WebClientResponseException {
        return logoutRequestCreation(logOutRequest);
    }
}
