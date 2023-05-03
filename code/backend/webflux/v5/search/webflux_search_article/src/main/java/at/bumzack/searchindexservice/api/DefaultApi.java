package at.bumzack.searchindexservice.api;

import at.bumzack.searchindexservice.ApiClient;
import at.bumzack.searchindexservice.model.MovieSearchResult;
import at.bumzack.searchindexservice.model.SearchMovieIndexRequest;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.ParameterizedTypeReference;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpMethod;
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

@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2023-04-29T15:35:07.759166+02:00[Europe/Vienna]")
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
     * @param searchMovieIndexRequest The searchMovieIndexRequest parameter
     * @return MovieSearchResult
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    private ResponseSpec searchArticleRequestCreation(SearchMovieIndexRequest searchMovieIndexRequest) throws WebClientResponseException {
        Object postBody = searchMovieIndexRequest;
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

        ParameterizedTypeReference<MovieSearchResult> localVarReturnType = new ParameterizedTypeReference<MovieSearchResult>() {
        };
        return apiClient.invokeAPI("/solr/v1/solr/searchindex/search", HttpMethod.POST, pathParams, queryParams, postBody, headerParams, cookieParams, formParams, localVarAccept, localVarContentType, localVarAuthNames, localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param searchMovieIndexRequest The searchMovieIndexRequest parameter
     * @return MovieSearchResult
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<MovieSearchResult> searchArticle(SearchMovieIndexRequest searchMovieIndexRequest) throws WebClientResponseException {
        ParameterizedTypeReference<MovieSearchResult> localVarReturnType = new ParameterizedTypeReference<MovieSearchResult>() {
        };
        return searchArticleRequestCreation(searchMovieIndexRequest).bodyToMono(localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param searchMovieIndexRequest The searchMovieIndexRequest parameter
     * @return ResponseEntity&lt;MovieSearchResult&gt;
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public Mono<ResponseEntity<MovieSearchResult>> searchArticleWithHttpInfo(SearchMovieIndexRequest searchMovieIndexRequest) throws WebClientResponseException {
        ParameterizedTypeReference<MovieSearchResult> localVarReturnType = new ParameterizedTypeReference<MovieSearchResult>() {
        };
        return searchArticleRequestCreation(searchMovieIndexRequest).toEntity(localVarReturnType);
    }

    /**
     * <p><b>200</b> - OK
     *
     * @param searchMovieIndexRequest The searchMovieIndexRequest parameter
     * @return ResponseSpec
     * @throws WebClientResponseException if an error occurs while attempting to invoke the API
     */
    public ResponseSpec searchArticleWithResponseSpec(SearchMovieIndexRequest searchMovieIndexRequest) throws WebClientResponseException {
        return searchArticleRequestCreation(searchMovieIndexRequest);
    }
}
