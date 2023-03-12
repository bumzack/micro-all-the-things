package at.bumzack.dto;

public class AbstractItem {

    private String id;
    private String code;

    public AbstractItem() {
    }

    public String getId() {
        return id;
    }

    public void setId(final String id) {
        this.id = id;
    }

    public String getCode() {
        return code;
    }

    public void setCode(final String code) {
        this.code = code;
    }

    @Override
    public String toString() {
        return "AbstractItem{" +
                "id='" + id + '\'' +
                ", code='" + code + '\'' +
                '}';
    }
}
