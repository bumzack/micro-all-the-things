package at.bumzack.foryouandyourfakewebshop.searchindexindex.model;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

import java.time.LocalDateTime;


@Table(name = "customer_price")
@Entity
public class CustomerPriceEntry {
    @Id
    @GeneratedValue
    private Long id;

    private Long customerId;
    private Double discount;
    private Integer startYear;
    private Integer endYear;
    private LocalDateTime created;

    public CustomerPriceEntry() {
    }

    public Long getId() {
        return id;
    }

    public void setId(final Long id) {
        this.id = id;
    }

    public Long getCustomerId() {
        return customerId;
    }

    public void setCustomerId(final Long customerId) {
        this.customerId = customerId;
    }

    public Double getDiscount() {
        return discount;
    }

    public void setDiscount(final Double discount) {
        this.discount = discount;
    }

    public Integer getStartYear() {
        return startYear;
    }

    public void setStartYear(final Integer startYear) {
        this.startYear = startYear;
    }

    public Integer getEndYear() {
        return endYear;
    }

    public void setEndYear(final Integer endYear) {
        this.endYear = endYear;
    }

    public LocalDateTime getCreated() {
        return created;
    }

    public void setCreated(final LocalDateTime created) {
        this.created = created;
    }

    @Override
    public String toString() {
        return "CustomerPriceEntry{" +
                "id=" + id +
                ", customerId=" + customerId +
                ", discount=" + discount +
                ", startYear=" + startYear +
                ", endYear=" + endYear +
                ", created=" + created +
                '}';
    }
}


