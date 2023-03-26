package at.bumzack.common.microthingisregistry;

import com.fasterxml.jackson.annotation.JsonProperty;

public class NewBackend {
    @JsonProperty("service_url")
    private String serviceUri;
    @JsonProperty("openapi_url")
    private String openapiUrl;
    @JsonProperty("local_repo_path")
    private String localRepoPath;
    @JsonProperty("microservice_id")
    private String microserviceId;
    @JsonProperty("technology_id")
    private int technologyId;
    @JsonProperty("api_client_prefix")
    private String apiClientPrefix;

    @JsonProperty("publish_as_frontend_package")
    private boolean publishAsFrontendPackage;

    @JsonProperty("api_client_package")
    private String apiClientPackage;


    public NewBackend() {
    }

    public String getServiceUri() {
        return serviceUri;
    }

    public void setServiceUri(String serviceUri) {
        this.serviceUri = serviceUri;
    }

    public String getOpenapiUrl() {
        return openapiUrl;
    }

    public void setOpenapiUrl(String openapiUrl) {
        this.openapiUrl = openapiUrl;
    }

    public String getLocalRepoPath() {
        return localRepoPath;
    }

    public void setLocalRepoPath(String localRepoPath) {
        this.localRepoPath = localRepoPath;
    }

    public String getMicroserviceId() {
        return microserviceId;
    }

    public void setMicroserviceId(String microserviceId) {
        this.microserviceId = microserviceId;
    }

    public int getTechnologyId() {
        return technologyId;
    }

    public void setTechnologyId(int technologyId) {
        this.technologyId = technologyId;
    }

    public boolean isPublishAsFrontendPackage() {
        return publishAsFrontendPackage;
    }

    public void setPublishAsFrontendPackage(boolean publishAsFrontendPackage) {
        this.publishAsFrontendPackage = publishAsFrontendPackage;
    }

    @Override
    public String toString() {
        return "NewBackend{" +
                "serviceUri='" + serviceUri + '\'' +
                ", openapiUrl='" + openapiUrl + '\'' +
                ", localRepoPath='" + localRepoPath + '\'' +
                ", microserviceId='" + microserviceId + '\'' +
                ", technologyId=" + technologyId +
                ", apiClientPrefix='" + apiClientPrefix + '\'' +
                ", publishAsFrontendPackage=" + publishAsFrontendPackage +
                ", apiClientPackage='" + apiClientPackage + '\'' +
                '}';
    }

    public String getApiClientPackage() {
        return apiClientPackage;
    }

    public void setApiClientPackage(String apiClientPackage) {
        this.apiClientPackage = apiClientPackage;
    }

    public String getApiClientPrefix() {
        return apiClientPrefix;
    }

    public void setApiClientPrefix(String apiClientPrefix) {
        this.apiClientPrefix = apiClientPrefix;
    }
}
