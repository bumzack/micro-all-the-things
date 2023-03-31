package at.bumzack.common.dto;

import java.io.Serializable;
import java.util.List;

public class TsvLine implements Serializable {

    private List<String> entries;

    private String original;

    public TsvLine() {
    }

    public List<String> getEntries() {
        return entries;
    }

    public void setEntries(final List<String> entries) {
        this.entries = entries;
    }

    public String getOriginal() {
        return original;
    }

    public void setOriginal(final String original) {
        this.original = original;
    }

    @Override
    public String toString() {
        return "TsvLine{" +
                "entries=" + entries +
                ", original='" + original + '\'' +
                '}';
    }
}
