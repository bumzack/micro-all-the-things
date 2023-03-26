package at.bumzack.common.search;


import at.bumzack.common.dto.AbstractItem;

import java.io.Serializable;
import java.util.List;
import java.util.stream.Collectors;


public class SearchResult<ITEM extends AbstractItem> implements Serializable {

    private static final long serialVersionUID = 1L;

    private List<ITEM> items;

    public SearchResult() {
    }

    public List<ITEM> getItems() {
        return items;
    }

    public void setItems(final List<ITEM> items) {
        this.items = items;
    }

    @Override
    public String toString() {
        return "SearchResult{" +
                "list=" + items.stream().map(AbstractItem::getCode).collect(Collectors.joining(" / ")) +
                '}';
    }
}
