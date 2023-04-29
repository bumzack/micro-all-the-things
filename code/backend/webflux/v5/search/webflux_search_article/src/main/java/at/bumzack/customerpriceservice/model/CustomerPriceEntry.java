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

import java.util.Objects;
import java.util.Arrays;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonTypeName;
import com.fasterxml.jackson.annotation.JsonValue;
import java.time.LocalDateTime;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;
import com.fasterxml.jackson.annotation.JsonTypeName;

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
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2023-04-29T15:35:12.132363+02:00[Europe/Vienna]")
public class CustomerPriceEntry {
  public static final String JSON_PROPERTY_ID = "id";
  private Long id;

  public static final String JSON_PROPERTY_CUSTOMER_ID = "customerId";
  private Long customerId;

  public static final String JSON_PROPERTY_DISCOUNT = "discount";
  private Double discount;

  public static final String JSON_PROPERTY_START_YEAR = "startYear";
  private Integer startYear;

  public static final String JSON_PROPERTY_END_YEAR = "endYear";
  private Integer endYear;

  public static final String JSON_PROPERTY_CREATED = "created";
  private LocalDateTime created;

  public CustomerPriceEntry() {
  }

  public CustomerPriceEntry id(Long id) {
    
    this.id = id;
    return this;
  }

   /**
   * Get id
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

}

