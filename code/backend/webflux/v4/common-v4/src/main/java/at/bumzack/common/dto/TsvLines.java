package at.bumzack.common.dto;

import java.io.Serializable;
import java.util.List;

public class TsvLines implements Serializable {

    private List<TsvLine> lines;


    public TsvLines() {
    }

    public List<TsvLine> getLines() {
        return lines;
    }

    public void setLines(List<TsvLine> lines) {
        this.lines = lines;
    }

    @Override
    public String toString() {
        return "TsvLines{" +
                "lines=" + lines +
                '}';
    }
}
