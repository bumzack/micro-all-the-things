package at.bumzack.common.solr;

import at.bumzack.common.dto.AbstractItem;
import at.bumzack.common.dto.SolrDocument;
import org.apache.commons.lang3.StringUtils;
import reactor.util.Logger;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class SolrReflectionThingi {
    private static final Logger LOG = reactor.util.Loggers.getLogger(SolrReflectionThingi.class);

    public static final List<String> getFieldNames(final Class<?> clazz) {
        if (!clazz.isAnnotationPresent(SolrDocument.class)) {
            throw new RuntimeException("The class " + clazz.getName() + " is missing the  SolrDocument annotation.");
        }
        // LOG.info("class  {}", clazz.getName());

        final var abstractClass = AbstractItem.class;
//        Arrays.stream(abstractClass.getDeclaredMethods()).filter(m -> m.getName().startsWith("set")).forEach(f -> LOG.info("setter method {}", f.getName()));
//        Arrays.stream(clazz.getDeclaredMethods()).filter(m -> m.getName().startsWith("set")).forEach(f -> LOG.info("setter method {}", f.getName()));

        final var res = new ArrayList<String>();

        final List<String> names = Arrays.stream(abstractClass.getDeclaredMethods())
                .filter(m -> m.getName().startsWith("set"))
                .map(m -> StringUtils.substring(m.getName(), 3))
                .map(name -> Character.toLowerCase(name.charAt(0)) + name.substring(1))
                .toList();

        final List<String> names2 = Arrays.stream(clazz.getDeclaredMethods())
                .filter(m -> m.getName().startsWith("set"))
                .map(m -> StringUtils.substring(m.getName(), 3))
                .map(name -> Character.toLowerCase(name.charAt(0)) + name.substring(1))
                .toList();

        res.addAll(names);
        res.addAll(names2);
        
        return res;
    }
}
