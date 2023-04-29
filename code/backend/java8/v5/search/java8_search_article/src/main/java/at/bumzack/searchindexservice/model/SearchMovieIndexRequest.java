/*
 * Search Index Index Service
 * Search Index Index Service
 *
 * The version of the OpenAPI document: 1.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


package at.bumzack.searchindexservice.model;

import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.util.StringJoiner;
import java.util.Objects;
import java.util.Arrays;
import java.util.Map;
import java.util.HashMap;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonTypeName;
import com.fasterxml.jackson.annotation.JsonValue;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


/**
 * SearchMovieIndexRequest
 */
@JsonPropertyOrder({
  SearchMovieIndexRequest.JSON_PROPERTY_Q,
  SearchMovieIndexRequest.JSON_PROPERTY_OFFSET,
  SearchMovieIndexRequest.JSON_PROPERTY_LIMIT
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2023-04-30T00:44:23.042256+02:00[Europe/Vienna]")
public class SearchMovieIndexRequest {
  public static final String JSON_PROPERTY_Q = "q";
  private String q;

  public static final String JSON_PROPERTY_OFFSET = "offset";
  private Integer offset;

  public static final String JSON_PROPERTY_LIMIT = "limit";
  private Integer limit;

  public SearchMovieIndexRequest() { 
  }

  public SearchMovieIndexRequest q(String q) {
    this.q = q;
    return this;
  }

   /**
   * Get q
   * @return q
  **/
  @javax.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_Q)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public String getQ() {
    return q;
  }


  @JsonProperty(JSON_PROPERTY_Q)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
  public void setQ(String q) {
    this.q = q;
  }


  public SearchMovieIndexRequest offset(Integer offset) {
    this.offset = offset;
    return this;
  }

   /**
   * Get offset
   * @return offset
  **/
  @javax.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_OFFSET)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public Integer getOffset() {
    return offset;
  }


  @JsonProperty(JSON_PROPERTY_OFFSET)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
  public void setOffset(Integer offset) {
    this.offset = offset;
  }


  public SearchMovieIndexRequest limit(Integer limit) {
    this.limit = limit;
    return this;
  }

   /**
   * Get limit
   * @return limit
  **/
  @javax.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_LIMIT)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public Integer getLimit() {
    return limit;
  }


  @JsonProperty(JSON_PROPERTY_LIMIT)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
  public void setLimit(Integer limit) {
    this.limit = limit;
  }


  /**
   * Return true if this SearchMovieIndexRequest object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    SearchMovieIndexRequest searchMovieIndexRequest = (SearchMovieIndexRequest) o;
    return Objects.equals(this.q, searchMovieIndexRequest.q) &&
        Objects.equals(this.offset, searchMovieIndexRequest.offset) &&
        Objects.equals(this.limit, searchMovieIndexRequest.limit);
  }

  @Override
  public int hashCode() {
    return Objects.hash(q, offset, limit);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class SearchMovieIndexRequest {\n");
    sb.append("    q: ").append(toIndentedString(q)).append("\n");
    sb.append("    offset: ").append(toIndentedString(offset)).append("\n");
    sb.append("    limit: ").append(toIndentedString(limit)).append("\n");
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

    // add `q` to the URL query string
    if (getQ() != null) {
      joiner.add(String.format("%sq%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getQ()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
    }

    // add `offset` to the URL query string
    if (getOffset() != null) {
      joiner.add(String.format("%soffset%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getOffset()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
    }

    // add `limit` to the URL query string
    if (getLimit() != null) {
      joiner.add(String.format("%slimit%s=%s", prefix, suffix, URLEncoder.encode(String.valueOf(getLimit()), StandardCharsets.UTF_8).replaceAll("\\+", "%20")));
    }

    return joiner.toString();
  }
}

