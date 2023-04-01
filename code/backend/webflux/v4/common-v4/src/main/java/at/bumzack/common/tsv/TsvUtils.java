package at.bumzack.common.tsv;

import com.google.common.base.CharMatcher;
import com.google.common.base.Splitter;
import reactor.util.Logger;
import reactor.util.Loggers;

import java.util.List;

public class TsvUtils {
    public static final Splitter SPLITTER_TSV = Splitter.on(CharMatcher.anyOf("\t"));
    public static final Splitter SPLITTER_ARRAY = Splitter.on(CharMatcher.anyOf(","));
    private static final Logger LOG = Loggers.getLogger(TsvUtils.class);
    public static final String N_A = "\\N";
    public static final String TRUE = "1";

    public static String getNullableValue(final String val) {
        return val.equals(N_A) ? null : val;
    }

    public static boolean getBoolean(final String val) {
        return val.equals(TRUE);
    }

    public static Integer getInteger(final String val) {
        return val.equals(N_A) ? null : Integer.parseInt(val);
    }

    public static Double getDouble(final String val) {
        return val.equals(N_A) ? null : Double.parseDouble(val);
    }

    public static List<String> getList(final String val) {
        //  LOG.info("list values {}, original value {}", StringUtils.join(" // ", strings), val);
        return val.equals(N_A) ? null : SPLITTER_ARRAY.splitToList(val);
    }
}
