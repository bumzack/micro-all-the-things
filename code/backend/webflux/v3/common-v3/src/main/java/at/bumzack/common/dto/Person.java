package at.bumzack.common.dto;

import at.bumzack.common.search.SearchResult;
import at.bumzack.common.solr.SolrResponse;
import org.apache.commons.lang3.StringUtils;
import org.springframework.core.ParameterizedTypeReference;

import java.io.Serializable;
import java.util.List;

@SolrDocument
public class Person extends AbstractItem implements Serializable {

    public static final ParameterizedTypeReference<SolrResponse<Person>> TYPE_REF_PERSON = new ParameterizedTypeReference<>() {
    };

    public static final ParameterizedTypeReference<SearchResult<Person>> TYPE_REF_SEARCH_RESULT_PERSON = new ParameterizedTypeReference<>() {
    };

    private static final long serialVersionUID = 3L;

    private String nconst;
    private String primaryName;
    private String birthYear;
    private String deathYear;
    private List<String> primaryProfession;
    private List<String> knownForTitles;

    public Person() {
    }

    public String getNconst() {
        return nconst;
    }

    public void setNconst(final String nconst) {
        this.nconst = nconst;
    }

    public String getPrimaryName() {
        return primaryName;
    }

    public void setPrimaryName(final String primaryName) {
        this.primaryName = primaryName;
    }

    public String getBirthYear() {
        return birthYear;
    }

    public void setBirthYear(final String birthYear) {
        this.birthYear = birthYear;
    }

    public String getDeathYear() {
        return deathYear;
    }

    public void setDeathYear(final String deathYear) {
        this.deathYear = deathYear;
    }

    public List<String> getPrimaryProfession() {
        return primaryProfession;
    }

    public void setPrimaryProfession(final List<String> primaryProfession) {
        this.primaryProfession = primaryProfession;
    }

    public List<String> getKnownForTitles() {
        return knownForTitles;
    }

    public void setKnownForTitles(final List<String> knownForTitles) {
        this.knownForTitles = knownForTitles;
    }

    @Override
    public String toString() {
        final var primaryProfessionStr = StringUtils.joinWith(" // ", primaryProfession);
        final var knownForTitlesStr = StringUtils.joinWith(" // ", knownForTitles);
        return "Person{" +
                "nconst='" + nconst + '\'' +
                ", id='" + getId() + '\'' +
                ", primaryName='" + primaryName + '\'' +
                ", birthYear='" + birthYear + '\'' +
                ", deathYear='" + deathYear + '\'' +
                ", primaryProfession=" + primaryProfessionStr +
                ", knownForTitles=" + knownForTitlesStr +
                '}';
    }
}
