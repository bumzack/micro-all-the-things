package at.bumzack.microthingisregistry;

import com.fasterxml.jackson.annotation.JsonProperty;

public class UpdateBackendOpenApi {

    @JsonProperty("openapiclient")
  private String openApiClient;

    public UpdateBackendOpenApi() {
    }

    public String getOpenApiClient() {
        return openApiClient;
    }

    public void setOpenApiClient(String openApiClient) {
        this.openApiClient = openApiClient;
    }

    @Override
    public String toString() {
        return "UpdateBackendOpenApi{" +
                "openApiClient='" + openApiClient + '\'' +
                '}';
    }
}
