package at.bumzack.microthingisregistry;

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

    @JsonProperty("publish_as_frontend_package")
    private boolean publishAsFrontendPackage;

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
                ", technologyId='" + technologyId + '\'' +
                ", publishAsFrontendPackage=" + publishAsFrontendPackage +
                '}';
    }
}
