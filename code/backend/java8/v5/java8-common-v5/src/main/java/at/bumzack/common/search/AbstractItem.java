package at.bumzack.common.search;

public class AbstractItem {

    private String id;

    public AbstractItem() {
    }

    public String getId() {
        return id;
    }

    public void setId(final String id) {
        this.id = id;
    }

    @Override
    public String toString() {
        return "AbstractItem{" +
                "id='" + id + '\'' +
                '}';
    }
}
