package at.bumzack.microthingisregistry;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Backend {
    @JsonProperty("id")
    private int id;

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

    @JsonProperty("openapiclient")
    private String openApiClient;

    @JsonProperty("host_id")
    private Integer hostId;


    public Backend() {
    }

    public int getId() {
        return id;
    }

    public void setId(int id) {
        this.id = id;
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

    public String getApiClientPrefix() {
        return apiClientPrefix;
    }

    public void setApiClientPrefix(String apiClientPrefix) {
        this.apiClientPrefix = apiClientPrefix;
    }

    public boolean isPublishAsFrontendPackage() {
        return publishAsFrontendPackage;
    }

    public void setPublishAsFrontendPackage(boolean publishAsFrontendPackage) {
        this.publishAsFrontendPackage = publishAsFrontendPackage;
    }

    public String getOpenApiClient() {
        return openApiClient;
    }

    public void setOpenApiClient(String openApiClient) {
        this.openApiClient = openApiClient;
    }

    public Integer getHostId() {
        return hostId;
    }

    public void setHostId(Integer hostId) {
        this.hostId = hostId;
    }

    @Override
    public String toString() {
        return "Backend{" +
                "id=" + id +
                ", serviceUri='" + serviceUri + '\'' +
                ", openapiUrl='" + openapiUrl + '\'' +
                ", localRepoPath='" + localRepoPath + '\'' +
                ", microserviceId='" + microserviceId + '\'' +
                ", technologyId=" + technologyId +
                ", apiClientPrefix='" + apiClientPrefix + '\'' +
                ", publishAsFrontendPackage=" + publishAsFrontendPackage +
                ", openApiClient='" + openApiClient + '\'' +
                ", hostId=" + hostId +
                '}';
    }
}
