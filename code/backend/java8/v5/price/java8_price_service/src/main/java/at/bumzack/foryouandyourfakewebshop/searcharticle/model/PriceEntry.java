package at.bumzack.foryouandyourfakewebshop.searcharticle.model;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

import java.time.LocalDateTime;
import java.util.Objects;


@Table(name = "price")
@Entity
public class PriceEntry {
    @Id
    @GeneratedValue
    private Long id;

    @Column(name = "movie_tconst")
    private String movieTconst;

    @Column(name = "amount")
    private Double amount;

    @Column(name = "created")
    private LocalDateTime created;

    public PriceEntry(Long id, String movieTconst, Double amount, LocalDateTime created) {
        this.id = id;
        this.movieTconst = movieTconst;
        this.amount = amount;
        this.created = created;
    }

    public PriceEntry() {

    }

    public Long getId() {
        return id;
    }

    public void setId(final Long id) {
        this.id = id;
    }

    public String getMovieTconst() {
        return movieTconst;
    }

    public void setMovieTconst(final String movieTconst) {
        this.movieTconst = movieTconst;
    }

    public Double getAmount() {
        return amount;
    }

    public void setAmount(final Double amount) {
        this.amount = amount;
    }

    public LocalDateTime getCreated() {
        return created;
    }

    public void setCreated(final LocalDateTime created) {
        this.created = created;
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == this) return true;
        if (obj == null || obj.getClass() != this.getClass()) return false;
        var that = (PriceEntry) obj;
        return Objects.equals(this.id, that.id) &&
                Objects.equals(this.movieTconst, that.movieTconst) &&
                Objects.equals(this.amount, that.amount) &&
                Objects.equals(this.created, that.created);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, movieTconst, amount, created);
    }

    @Override
    public String toString() {
        return "PriceEntry[" +
                "id=" + id + ", " +
                "movieTconst=" + movieTconst + ", " +
                "amount=" + amount + ", " +
                "created=" + created + ']';
    }

}


