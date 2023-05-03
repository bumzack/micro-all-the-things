/*
 * Authentication Service
 * Authentication Service
 *
 * The version of the OpenAPI document: 1.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

package at.bumzack.authenticationservice.api;

import at.bumzack.authenticationservice.ApiClient;
import at.bumzack.authenticationservice.ApiException;
import at.bumzack.authenticationservice.ApiResponse;
import at.bumzack.authenticationservice.model.AuthenticationEntry;
import at.bumzack.authenticationservice.model.LogInRequest;
import at.bumzack.authenticationservice.model.LogOutRequest;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.IOException;
import java.io.InputStream;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.time.Duration;
import java.util.function.Consumer;

@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2023-04-30T00:44:18.352755+02:00[Europe/Vienna]")
public class DefaultApi {
    private final HttpClient memberVarHttpClient;
    private final ObjectMapper memberVarObjectMapper;
    private final String memberVarBaseUri;
    private final Consumer<HttpRequest.Builder> memberVarInterceptor;
    private final Duration memberVarReadTimeout;
    private final Consumer<HttpResponse<InputStream>> memberVarResponseInterceptor;
    private final Consumer<HttpResponse<String>> memberVarAsyncResponseInterceptor;

    public DefaultApi() {
        this(new ApiClient());
    }

    public DefaultApi(ApiClient apiClient) {
        memberVarHttpClient = apiClient.getHttpClient();
        memberVarObjectMapper = apiClient.getObjectMapper();
        memberVarBaseUri = apiClient.getBaseUri();
        memberVarInterceptor = apiClient.getRequestInterceptor();
        memberVarReadTimeout = apiClient.getReadTimeout();
        memberVarResponseInterceptor = apiClient.getResponseInterceptor();
        memberVarAsyncResponseInterceptor = apiClient.getAsyncResponseInterceptor();
    }

    protected ApiException getApiException(String operationId, HttpResponse<InputStream> response) throws IOException {
        String body = response.body() == null ? null : new String(response.body().readAllBytes());
        String message = formatExceptionMessage(operationId, response.statusCode(), body);
        return new ApiException(response.statusCode(), message, response.headers(), body);
    }

    private String formatExceptionMessage(String operationId, int statusCode, String body) {
        if (body == null || body.isEmpty()) {
            body = "[no body]";
        }
        return operationId + " call failed with: " + statusCode + " - " + body;
    }

    /**
     * @param customerId Customer ID (required)
     * @return AuthenticationEntry
     * @throws ApiException if fails to make API call
     */
    public AuthenticationEntry loggedin(Long customerId) throws ApiException {
        ApiResponse<AuthenticationEntry> localVarResponse = loggedinWithHttpInfo(customerId);
        return localVarResponse.getData();
    }

    /**
     * @param customerId Customer ID (required)
     * @return ApiResponse&lt;AuthenticationEntry&gt;
     * @throws ApiException if fails to make API call
     */
    public ApiResponse<AuthenticationEntry> loggedinWithHttpInfo(Long customerId) throws ApiException {
        HttpRequest.Builder localVarRequestBuilder = loggedinRequestBuilder(customerId);
        try {
            HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
                    localVarRequestBuilder.build(),
                    HttpResponse.BodyHandlers.ofInputStream());
            if (memberVarResponseInterceptor != null) {
                memberVarResponseInterceptor.accept(localVarResponse);
            }
            try {
                if (localVarResponse.statusCode() / 100 != 2) {
                    throw getApiException("loggedin", localVarResponse);
                }
                return new ApiResponse<AuthenticationEntry>(
                        localVarResponse.statusCode(),
                        localVarResponse.headers().map(),
                        localVarResponse.body() == null ? null : memberVarObjectMapper.readValue(localVarResponse.body(), new TypeReference<AuthenticationEntry>() {
                        }) // closes the InputStream
                );
            } finally {
            }
        } catch (IOException e) {
            throw new ApiException(e);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            throw new ApiException(e);
        }
    }

    private HttpRequest.Builder loggedinRequestBuilder(Long customerId) throws ApiException {
        // verify the required parameter 'customerId' is set
        if (customerId == null) {
            throw new ApiException(400, "Missing the required parameter 'customerId' when calling loggedin");
        }

        HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

        String localVarPath = "/api/v1/authenticated/{customerId}"
                .replace("{customerId}", ApiClient.urlEncode(customerId.toString()));

        localVarRequestBuilder.uri(URI.create(memberVarBaseUri + localVarPath));

        localVarRequestBuilder.header("Accept", "*/*");

        localVarRequestBuilder.method("GET", HttpRequest.BodyPublishers.noBody());
        if (memberVarReadTimeout != null) {
            localVarRequestBuilder.timeout(memberVarReadTimeout);
        }
        if (memberVarInterceptor != null) {
            memberVarInterceptor.accept(localVarRequestBuilder);
        }
        return localVarRequestBuilder;
    }

    /**
     * @param logInRequest (optional)
     * @return AuthenticationEntry
     * @throws ApiException if fails to make API call
     */
    public AuthenticationEntry login(LogInRequest logInRequest) throws ApiException {
        ApiResponse<AuthenticationEntry> localVarResponse = loginWithHttpInfo(logInRequest);
        return localVarResponse.getData();
    }

    /**
     * @param logInRequest (optional)
     * @return ApiResponse&lt;AuthenticationEntry&gt;
     * @throws ApiException if fails to make API call
     */
    public ApiResponse<AuthenticationEntry> loginWithHttpInfo(LogInRequest logInRequest) throws ApiException {
        HttpRequest.Builder localVarRequestBuilder = loginRequestBuilder(logInRequest);
        try {
            HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
                    localVarRequestBuilder.build(),
                    HttpResponse.BodyHandlers.ofInputStream());
            if (memberVarResponseInterceptor != null) {
                memberVarResponseInterceptor.accept(localVarResponse);
            }
            try {
                if (localVarResponse.statusCode() / 100 != 2) {
                    throw getApiException("login", localVarResponse);
                }
                return new ApiResponse<AuthenticationEntry>(
                        localVarResponse.statusCode(),
                        localVarResponse.headers().map(),
                        localVarResponse.body() == null ? null : memberVarObjectMapper.readValue(localVarResponse.body(), new TypeReference<AuthenticationEntry>() {
                        }) // closes the InputStream
                );
            } finally {
            }
        } catch (IOException e) {
            throw new ApiException(e);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            throw new ApiException(e);
        }
    }

    private HttpRequest.Builder loginRequestBuilder(LogInRequest logInRequest) throws ApiException {

        HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

        String localVarPath = "/api/v1/authentication/login";

        localVarRequestBuilder.uri(URI.create(memberVarBaseUri + localVarPath));

        localVarRequestBuilder.header("Content-Type", "application/json");
        localVarRequestBuilder.header("Accept", "*/*");

        try {
            byte[] localVarPostBody = memberVarObjectMapper.writeValueAsBytes(logInRequest);
            localVarRequestBuilder.method("POST", HttpRequest.BodyPublishers.ofByteArray(localVarPostBody));
        } catch (IOException e) {
            throw new ApiException(e);
        }
        if (memberVarReadTimeout != null) {
            localVarRequestBuilder.timeout(memberVarReadTimeout);
        }
        if (memberVarInterceptor != null) {
            memberVarInterceptor.accept(localVarRequestBuilder);
        }
        return localVarRequestBuilder;
    }

    /**
     * @param logOutRequest (optional)
     * @return AuthenticationEntry
     * @throws ApiException if fails to make API call
     */
    public AuthenticationEntry logout(LogOutRequest logOutRequest) throws ApiException {
        ApiResponse<AuthenticationEntry> localVarResponse = logoutWithHttpInfo(logOutRequest);
        return localVarResponse.getData();
    }

    /**
     * @param logOutRequest (optional)
     * @return ApiResponse&lt;AuthenticationEntry&gt;
     * @throws ApiException if fails to make API call
     */
    public ApiResponse<AuthenticationEntry> logoutWithHttpInfo(LogOutRequest logOutRequest) throws ApiException {
        HttpRequest.Builder localVarRequestBuilder = logoutRequestBuilder(logOutRequest);
        try {
            HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
                    localVarRequestBuilder.build(),
                    HttpResponse.BodyHandlers.ofInputStream());
            if (memberVarResponseInterceptor != null) {
                memberVarResponseInterceptor.accept(localVarResponse);
            }
            try {
                if (localVarResponse.statusCode() / 100 != 2) {
                    throw getApiException("logout", localVarResponse);
                }
                return new ApiResponse<AuthenticationEntry>(
                        localVarResponse.statusCode(),
                        localVarResponse.headers().map(),
                        localVarResponse.body() == null ? null : memberVarObjectMapper.readValue(localVarResponse.body(), new TypeReference<AuthenticationEntry>() {
                        }) // closes the InputStream
                );
            } finally {
            }
        } catch (IOException e) {
            throw new ApiException(e);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            throw new ApiException(e);
        }
    }

    private HttpRequest.Builder logoutRequestBuilder(LogOutRequest logOutRequest) throws ApiException {

        HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

        String localVarPath = "/api/v1/authentication/logout";

        localVarRequestBuilder.uri(URI.create(memberVarBaseUri + localVarPath));

        localVarRequestBuilder.header("Content-Type", "application/json");
        localVarRequestBuilder.header("Accept", "*/*");

        try {
            byte[] localVarPostBody = memberVarObjectMapper.writeValueAsBytes(logOutRequest);
            localVarRequestBuilder.method("POST", HttpRequest.BodyPublishers.ofByteArray(localVarPostBody));
        } catch (IOException e) {
            throw new ApiException(e);
        }
        if (memberVarReadTimeout != null) {
            localVarRequestBuilder.timeout(memberVarReadTimeout);
        }
        if (memberVarInterceptor != null) {
            memberVarInterceptor.accept(localVarRequestBuilder);
        }
        return localVarRequestBuilder;
    }
}
