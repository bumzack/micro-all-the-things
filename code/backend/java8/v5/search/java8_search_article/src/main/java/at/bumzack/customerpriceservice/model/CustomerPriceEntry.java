/*
 * CustomerPrice Service
 * CustomerPrice Service
 *
 * The version of the OpenAPI document: 1.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


package at.bumzack.customerpriceservice.model;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;

import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.time.LocalDateTime;
import java.util.Objects;
import java.util.StringJoiner;


/**
 * CustomerPriceEntry
 */
@JsonPropertyOrder({
        CustomerPriceEntry.JSON_PROPERTY_ID,
        CustomerPriceEntry.JSON_PROPERTY_CUSTOMER_ID,
        CustomerPriceEntry.JSON_PROPERTY_DISCOUNT,
        CustomerPriceEntry.JSON_PROPERTY_START_YEAR,
        CustomerPriceEntry.JSON_PROPERTY_END_YEAR,
        CustomerPriceEntry.JSON_PROPERTY_CREATED
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2023-04-30T01:18:26.210378+02:00[Europe/Vienna]")
public class CustomerPriceEntry {
    public static final String JSON_PROPERTY_ID = "id";
    public static final String JSON_PROPERTY_CUSTOMER_ID = "customerId";
    public static final String JSON_PROPERTY_DISCOUNT = "discount";
    public static final String JSON_PROPERTY_START_YEAR = "startYear";
    public static final String JSON_PROPERTY_END_YEAR = "endYear";
    public static final String JSON_PROPERTY_CREATED = "created";
    private Long id;
    private Long customerId;
    private Double discount;
    private Integer startYear;
    private Integer endYear;
    private LocalDateTime created;

    public CustomerPriceEntry() {
    }

    public CustomerPriceEntry id(Long id) {
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


    public CustomerPriceEntry customerId(Long customerId) {
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


    public CustomerPriceEntry discount(Double discount) {
        this.discount = discount;
        return this;
    }

    /**
     * Get discount
     *
     * @return discount
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_DISCOUNT)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public Double getDiscount() {
        return discount;
    }


    @JsonProperty(JSON_PROPERTY_DISCOUNT)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setDiscount(Double discount) {
        this.discount = discount;
    }


    public CustomerPriceEntry startYear(Integer startYear) {
        this.startYear = startYear;
        return this;
    }

    /**
     * Get startYear
     *
     * @return startYear
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_START_YEAR)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public Integer getStartYear() {
        return startYear;
    }


    @JsonProperty(JSON_PROPERTY_START_YEAR)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setStartYear(Integer startYear) {
        this.startYear = startYear;
    }


    public CustomerPriceEntry endYear(Integer endYear) {
        this.endYear = endYear;
        return this;
    }

    /**
     * Get endYear
     *
     * @return endYear
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_END_YEAR)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public Integer getEndYear() {
        return endYear;
    }


    @JsonProperty(JSON_PROPERTY_END_YEAR)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setEndYear(Integer endYear) {
        this.endYear = endYear;
    }


    public CustomerPriceEntry created(LocalDateTime created) {
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
     * Return true if this CustomerPriceEntry object is equal to o.
     */
    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
        CustomerPriceEntry customerPriceEntry = (CustomerPriceEntry) o;
        return Objects.equals(this.id, customerPriceEntry.id) &&
                Objects.equals(this.customerId, customerPriceEntry.customerId) &&
                Objects.equals(this.discount, customerPriceEntry.discount) &&
                Objects.equals(this.startYear, customerPriceEntry.startYear) &&
                Objects.equals(this.endYear, customerPriceEntry.endYear) &&
                Objects.equals(this.created, customerPriceEntry.created);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, customerId, discount, startYear, endYear, created);
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append("class CustomerPriceEntry {\n");
        sb.append("    id: ").append(toIndentedString(id)).append("\n");
        sb.append("    customerId: ").append(toIndentedString(customerId)).append("\n");
        sb.append("    discount: ").append(toIndentedString(discount)).append("\n");
        sb.append("    startYear: ").append(toIndentedString(startYear)).append("\n");
        sb.append("    endYear: ").append(toIndentedString(endYear)).append("\n");
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

        // add `discount` to the URL query string
        if (getDiscount() != null) {
            joiner.add(String.format("%sdiscount%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getDiscount()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
        }

        // add `startYear` to the URL query string
        if (getStartYear() != null) {
            joiner.add(String.format("%sstartYear%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getStartYear()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
        }

        // add `endYear` to the URL query string
        if (getEndYear() != null) {
            joiner.add(String.format("%sendYear%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getEndYear()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
        }

        // add `created` to the URL query string
        if (getCreated() != null) {
            joiner.add(String.format("%screated%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getCreated()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
        }

        return joiner.toString();
    }
}

