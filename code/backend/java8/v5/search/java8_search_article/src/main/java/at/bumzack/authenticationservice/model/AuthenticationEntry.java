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

import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.time.LocalDateTime;
import java.util.Objects;
import java.util.StringJoiner;


/**
 * AuthenticationEntry
 */
@JsonPropertyOrder({
        AuthenticationEntry.JSON_PROPERTY_ID,
        AuthenticationEntry.JSON_PROPERTY_CUSTOMER_ID,
        AuthenticationEntry.JSON_PROPERTY_JWT,
        AuthenticationEntry.JSON_PROPERTY_LOGGED_IN,
        AuthenticationEntry.JSON_PROPERTY_LOGGED_OUT,
        AuthenticationEntry.JSON_PROPERTY_CREATED
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2023-04-30T00:44:18.352755+02:00[Europe/Vienna]")
public class AuthenticationEntry {
    public static final String JSON_PROPERTY_ID = "id";
    public static final String JSON_PROPERTY_CUSTOMER_ID = "customerId";
    public static final String JSON_PROPERTY_JWT = "jwt";
    public static final String JSON_PROPERTY_LOGGED_IN = "loggedIn";
    public static final String JSON_PROPERTY_LOGGED_OUT = "loggedOut";
    public static final String JSON_PROPERTY_CREATED = "created";
    private Long id;
    private Long customerId;
    private String jwt;
    private LocalDateTime loggedIn;
    private LocalDateTime loggedOut;
    private LocalDateTime created;

    public AuthenticationEntry() {
    }

    public AuthenticationEntry id(Long id) {
        this.id = id;
        return this;
    }

    /**
     * Get id
     *
     * @return id
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_ID)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public Long getId() {
        return id;
    }


    @JsonProperty(JSON_PROPERTY_ID)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setId(Long id) {
        this.id = id;
    }


    public AuthenticationEntry customerId(Long customerId) {
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


    public AuthenticationEntry jwt(String jwt) {
        this.jwt = jwt;
        return this;
    }

    /**
     * Get jwt
     *
     * @return jwt
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_JWT)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public String getJwt() {
        return jwt;
    }


    @JsonProperty(JSON_PROPERTY_JWT)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setJwt(String jwt) {
        this.jwt = jwt;
    }


    public AuthenticationEntry loggedIn(LocalDateTime loggedIn) {
        this.loggedIn = loggedIn;
        return this;
    }

    /**
     * Get loggedIn
     *
     * @return loggedIn
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_LOGGED_IN)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public LocalDateTime getLoggedIn() {
        return loggedIn;
    }


    @JsonProperty(JSON_PROPERTY_LOGGED_IN)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setLoggedIn(LocalDateTime loggedIn) {
        this.loggedIn = loggedIn;
    }


    public AuthenticationEntry loggedOut(LocalDateTime loggedOut) {
        this.loggedOut = loggedOut;
        return this;
    }

    /**
     * Get loggedOut
     *
     * @return loggedOut
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_LOGGED_OUT)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public LocalDateTime getLoggedOut() {
        return loggedOut;
    }


    @JsonProperty(JSON_PROPERTY_LOGGED_OUT)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setLoggedOut(LocalDateTime loggedOut) {
        this.loggedOut = loggedOut;
    }


    public AuthenticationEntry created(LocalDateTime created) {
        this.created = created;
        return this;
    }

    /**
     * Get created
     *
     * @return created
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_CREATED)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public LocalDateTime getCreated() {
        return created;
    }


    @JsonProperty(JSON_PROPERTY_CREATED)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setCreated(LocalDateTime created) {
        this.created = created;
    }


    /**
     * Return true if this AuthenticationEntry object is equal to o.
     */
    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
        AuthenticationEntry authenticationEntry = (AuthenticationEntry) o;
        return Objects.equals(this.id, authenticationEntry.id) &&
                Objects.equals(this.customerId, authenticationEntry.customerId) &&
                Objects.equals(this.jwt, authenticationEntry.jwt) &&
                Objects.equals(this.loggedIn, authenticationEntry.loggedIn) &&
                Objects.equals(this.loggedOut, authenticationEntry.loggedOut) &&
                Objects.equals(this.created, authenticationEntry.created);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, customerId, jwt, loggedIn, loggedOut, created);
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append("class AuthenticationEntry {\n");
        sb.append("    id: ").append(toIndentedString(id)).append("\n");
        sb.append("    customerId: ").append(toIndentedString(customerId)).append("\n");
        sb.append("    jwt: ").append(toIndentedString(jwt)).append("\n");
        sb.append("    loggedIn: ").append(toIndentedString(loggedIn)).append("\n");
        sb.append("    loggedOut: ").append(toIndentedString(loggedOut)).append("\n");
        sb.append("    created: ").append(toIndentedString(created)).append("\n");
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

    /**
     * Convert the instance into URL query string.
     *
     * @return URL query string
     */
    public String toUrlQueryString() {
        return toUrlQueryString(null);
    }

    /**
     * Convert the instance into URL query string.
     *
     * @param prefix prefix of the query string
     * @return URL query string
     */
    public String toUrlQueryString(String prefix) {
        String suffix = "";
        String containerSuffix = "";
        String containerPrefix = "";
        if (prefix == null) {
            // style=form, explode=true, e.g. /pet?name=cat&type=manx
            prefix = "";
        } else {
            // deepObject style e.g. /pet?id[name]=cat&id[type]=manx
            prefix = prefix + "[";
            suffix = "]";
            containerSuffix = "]";
            containerPrefix = "[";
        }

        StringJoiner joiner = new StringJoiner("&");

        // add `id` to the URL query string
        if (getId() != null) {
            joiner.add(String.format("%sid%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getId()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
        }

        // add `customerId` to the URL query string
        if (getCustomerId() != null) {
            joiner.add(String.format("%scustomerId%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getCustomerId()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
        }

        // add `jwt` to the URL query string
        if (getJwt() != null) {
            joiner.add(String.format("%sjwt%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getJwt()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
        }

        // add `loggedIn` to the URL query string
        if (getLoggedIn() != null) {
            joiner.add(String.format("%sloggedIn%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getLoggedIn()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
        }

        // add `loggedOut` to the URL query string
        if (getLoggedOut() != null) {
            joiner.add(String.format("%sloggedOut%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getLoggedOut()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
        }

        // add `created` to the URL query string
        if (getCreated() != null) {
            joiner.add(String.format("%screated%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getCreated()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
        }

        return joiner.toString();
    }
}

