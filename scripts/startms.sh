

for f in solr*; do
    if [ -d "$f" ]; then
        cd $f
        echo "folder $f"
        mvn spring-boot:run &
        cd ..
    fi
done

for f in tsv*; do
    if [ -d "$f" ]; then
        cd $f
        mvn spring-boot:run &
        cd ..
    fi
done
