package at.bumzack.priceservice.api;

import at.bumzack.priceservice.ApiClient;

import at.bumzack.priceservice.model.PriceEntry;
import at.bumzack.priceservice.model.SearchPricesRequest;

import java.util.HashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.stream.Collectors;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.util.LinkedMultiValueMap;
import org.springframework.util.MultiValueMap;
import org.springframework.core.ParameterizedTypeReference;
import org.springframework.web.reactive.function.client.WebClient.ResponseSpec;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.core.io.FileSystemResource;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpMethod;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import reactor.core.publisher.Mono;
import reactor.core.publisher.Flux;

@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2023-04-29T15:35:20.403886+02:00[Europe/Vienna]")
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
     * 
     * 
     * <p><b>200</b> - OK
     * @param tconst Movie tconst ID
     * @return PriceEntry
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    private ResponseSpec getPriceForMovieRequestCreation(String tconst) throws WebClientResponseException {
        Object postBody = null;
        // verify the required parameter 'tconst' is set
        if (tconst == null) {
            throw new WebClientResponseException("Missing the required parameter 'tconst' when calling getPriceForMovie", HttpStatus.BAD_REQUEST.value(), HttpStatus.BAD_REQUEST.getReasonPhrase(), null, null, null);
        }
        // create path and map variables
        final Map<String, Object> pathParams = new HashMap<String, Object>();

        pathParams.put("tconst", tconst);

        final MultiValueMap<String, String> queryParams = new LinkedMultiValueMap<String, String>();
        final HttpHeaders headerParams = new HttpHeaders();
        final MultiValueMap<String, String> cookieParams = new LinkedMultiValueMap<String, String>();
        final MultiValueMap<String, Object> formParams = new LinkedMultiValueMap<String, Object>();

        final String[] localVarAccepts = { 
            "*/*"
        };
        final List<MediaType> localVarAccept = apiClient.selectHeaderAccept(localVarAccepts);
        final String[] localVarContentTypes = { };
        final MediaType localVarContentType = apiClient.selectHeaderContentType(localVarContentTypes);

        String[] localVarAuthNames = new String[] {  };

        ParameterizedTypeReference<PriceEntry> localVarReturnType = new ParameterizedTypeReference<PriceEntry>() {};
        return apiClient.invokeAPI("/api/v1/price/{tconst}", HttpMethod.GET, pathParams, queryParams, postBody, headerParams, cookieParams, formParams, localVarAccept, localVarContentType, localVarAuthNames, localVarReturnType);
    }

    /**
     * 
     * 
     * <p><b>200</b> - OK
     * @param tconst Movie tconst ID
     * @return PriceEntry
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<PriceEntry> getPriceForMovie(String tconst) throws WebClientResponseException {
        ParameterizedTypeReference<PriceEntry> localVarReturnType = new ParameterizedTypeReference<PriceEntry>() {};
        return getPriceForMovieRequestCreation(tconst).bodyToMono(localVarReturnType);
    }

    /**
     * 
     * 
     * <p><b>200</b> - OK
     * @param tconst Movie tconst ID
     * @return ResponseEntity&lt;PriceEntry&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<ResponseEntity<PriceEntry>> getPriceForMovieWithHttpInfo(String tconst) throws WebClientResponseException {
        ParameterizedTypeReference<PriceEntry> localVarReturnType = new ParameterizedTypeReference<PriceEntry>() {};
        return getPriceForMovieRequestCreation(tconst).toEntity(localVarReturnType);
    }

    /**
     * 
     * 
     * <p><b>200</b> - OK
     * @param tconst Movie tconst ID
     * @return ResponseSpec
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public ResponseSpec getPriceForMovieWithResponseSpec(String tconst) throws WebClientResponseException {
        return getPriceForMovieRequestCreation(tconst);
    }
    /**
     * 
     * 
     * <p><b>200</b> - OK
     * @param searchPricesRequest The searchPricesRequest parameter
     * @return List&lt;PriceEntry&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    private ResponseSpec getPricesForMoviesRequestCreation(SearchPricesRequest searchPricesRequest) throws WebClientResponseException {
        Object postBody = searchPricesRequest;
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
        final String[] localVarContentTypes = { };
        final MediaType localVarContentType = apiClient.selectHeaderContentType(localVarContentTypes);

        String[] localVarAuthNames = new String[] {  };

        ParameterizedTypeReference<PriceEntry> localVarReturnType = new ParameterizedTypeReference<PriceEntry>() {};
        return apiClient.invokeAPI("/api/v2/prices", HttpMethod.POST, pathParams, queryParams, postBody, headerParams, cookieParams, formParams, localVarAccept, localVarContentType, localVarAuthNames, localVarReturnType);
    }

    /**
     * 
     * 
     * <p><b>200</b> - OK
     * @param searchPricesRequest The searchPricesRequest parameter
     * @return List&lt;PriceEntry&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Flux<PriceEntry> getPricesForMovies(SearchPricesRequest searchPricesRequest) throws WebClientResponseException {
        ParameterizedTypeReference<PriceEntry> localVarReturnType = new ParameterizedTypeReference<PriceEntry>() {};
        return getPricesForMoviesRequestCreation(searchPricesRequest).bodyToFlux(localVarReturnType);
    }

    /**
     * 
     * 
     * <p><b>200</b> - OK
     * @param searchPricesRequest The searchPricesRequest parameter
     * @return ResponseEntity&lt;List&lt;PriceEntry&gt;&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<ResponseEntity<List<PriceEntry>>> getPricesForMoviesWithHttpInfo(SearchPricesRequest searchPricesRequest) throws WebClientResponseException {
        ParameterizedTypeReference<PriceEntry> localVarReturnType = new ParameterizedTypeReference<PriceEntry>() {};
        return getPricesForMoviesRequestCreation(searchPricesRequest).toEntityList(localVarReturnType);
    }

    /**
     * 
     * 
     * <p><b>200</b> - OK
     * @param searchPricesRequest The searchPricesRequest parameter
     * @return ResponseSpec
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public ResponseSpec getPricesForMoviesWithResponseSpec(SearchPricesRequest searchPricesRequest) throws WebClientResponseException {
        return getPricesForMoviesRequestCreation(searchPricesRequest);
    }
}
