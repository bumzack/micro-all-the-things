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


package at.bumzack.authenticationservice.model;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;

import java.util.Objects;

/**
 * LogOutRequest
 */
@JsonPropertyOrder({
        LogOutRequest.JSON_PROPERTY_CUSTOMER_ID
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2023-04-29T15:35:02.276511+02:00[Europe/Vienna]")
public class LogOutRequest {
    public static final String JSON_PROPERTY_CUSTOMER_ID = "customerId";
    private Long customerId;

    public LogOutRequest() {
    }

    public LogOutRequest customerId(Long customerId) {

        this.customerId = customerId;
        return this;
    }

    /**
     * Get customerId
     *
     * @return customerId
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_CUSTOMER_ID)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public Long getCustomerId() {
        return customerId;
    }


    @JsonProperty(JSON_PROPERTY_CUSTOMER_ID)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setCustomerId(Long customerId) {
        this.customerId = customerId;
    }


    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
        LogOutRequest logOutRequest = (LogOutRequest) o;
        return Objects.equals(this.customerId, logOutRequest.customerId);
    }

    @Override
    public int hashCode() {
        return Objects.hash(customerId);
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append("class LogOutRequest {\n");
        sb.append("    customerId: ").append(toIndentedString(customerId)).append("\n");
        sb.append("}");
        return sb.toString();
    }

    /**
     * Convert the given object to string with each line indented by 4 spaces
     * (except the first line).
     */
    private String toIndentedString(Object o) {
        if (o == null) {
            return "null";
        }
        return o.toString().replace("\n", "\n    ");
    }

}

