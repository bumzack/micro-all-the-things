/*
 * Customer Service
 * Customer Service
 *
 * The version of the OpenAPI document: 1.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


package at.bumzack.customerservice.model;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;

import java.time.OffsetDateTime;
import java.util.Objects;

/**
 * Customer
 */
@JsonPropertyOrder({
        Customer.JSON_PROPERTY_ID,
        Customer.JSON_PROPERTY_FIRST_NAME,
        Customer.JSON_PROPERTY_LAST_NAME,
        Customer.JSON_PROPERTY_EMAIL,
        Customer.JSON_PROPERTY_PASSWORD,
        Customer.JSON_PROPERTY_CREATEED
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2023-04-28T18:36:13.086172+02:00[Europe/Vienna]")
public class Customer {
    public static final String JSON_PROPERTY_ID = "id";
    public static final String JSON_PROPERTY_FIRST_NAME = "firstName";
    public static final String JSON_PROPERTY_LAST_NAME = "lastName";
    public static final String JSON_PROPERTY_EMAIL = "email";
    public static final String JSON_PROPERTY_PASSWORD = "password";
    public static final String JSON_PROPERTY_CREATEED = "createed";
    private Long id;
    private String firstName;
    private String lastName;
    private String email;
    private String password;
    private OffsetDateTime createed;

    public Customer() {
    }

    public Customer id(Long id) {

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


    public Customer firstName(String firstName) {

        this.firstName = firstName;
        return this;
    }

    /**
     * Get firstName
     *
     * @return firstName
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_FIRST_NAME)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public String getFirstName() {
        return firstName;
    }


    @JsonProperty(JSON_PROPERTY_FIRST_NAME)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setFirstName(String firstName) {
        this.firstName = firstName;
    }


    public Customer lastName(String lastName) {

        this.lastName = lastName;
        return this;
    }

    /**
     * Get lastName
     *
     * @return lastName
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_LAST_NAME)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public String getLastName() {
        return lastName;
    }


    @JsonProperty(JSON_PROPERTY_LAST_NAME)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setLastName(String lastName) {
        this.lastName = lastName;
    }


    public Customer email(String email) {

        this.email = email;
        return this;
    }

    /**
     * Get email
     *
     * @return email
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_EMAIL)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public String getEmail() {
        return email;
    }


    @JsonProperty(JSON_PROPERTY_EMAIL)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setEmail(String email) {
        this.email = email;
    }


    public Customer password(String password) {

        this.password = password;
        return this;
    }

    /**
     * Get password
     *
     * @return password
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_PASSWORD)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public String getPassword() {
        return password;
    }


    @JsonProperty(JSON_PROPERTY_PASSWORD)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setPassword(String password) {
        this.password = password;
    }


    public Customer createed(OffsetDateTime createed) {

        this.createed = createed;
        return this;
    }

    /**
     * Get createed
     *
     * @return createed
     **/
    @javax.annotation.Nullable
    @JsonProperty(JSON_PROPERTY_CREATEED)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

    public OffsetDateTime getCreateed() {
        return createed;
    }


    @JsonProperty(JSON_PROPERTY_CREATEED)
    @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
    public void setCreateed(OffsetDateTime createed) {
        this.createed = createed;
    }


    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
        Customer customer = (Customer) o;
        return Objects.equals(this.id, customer.id) &&
                Objects.equals(this.firstName, customer.firstName) &&
                Objects.equals(this.lastName, customer.lastName) &&
                Objects.equals(this.email, customer.email) &&
                Objects.equals(this.password, customer.password) &&
                Objects.equals(this.createed, customer.createed);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, firstName, lastName, email, password, createed);
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append("class Customer {\n");
        sb.append("    id: ").append(toIndentedString(id)).append("\n");
        sb.append("    firstName: ").append(toIndentedString(firstName)).append("\n");
        sb.append("    lastName: ").append(toIndentedString(lastName)).append("\n");
        sb.append("    email: ").append(toIndentedString(email)).append("\n");
        sb.append("    password: ").append(toIndentedString(password)).append("\n");
        sb.append("    createed: ").append(toIndentedString(createed)).append("\n");
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

