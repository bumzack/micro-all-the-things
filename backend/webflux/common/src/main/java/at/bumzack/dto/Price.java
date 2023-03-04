package at.rwa.saphy.portal.webservices.controllers;

import java.math.BigDecimal;

public class PriceData {

    private String code;
    private BigDecimal price;


    public PriceData() {
    }

    public String getCode() {
        return code;
    }

    public void setCode(final String code) {
        this.code = code;
    }

    public BigDecimal getPrice() {
        return price;
    }

    public void setPrice(final BigDecimal price) {
        this.price = price;
    }

    @Override
    public String toString() {
        return "PriceData{" +
                "code='" + code + '\'' +
                ", price=" + price +
                '}';
    }
}
