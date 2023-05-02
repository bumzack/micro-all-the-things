FROM maven:3.9.1-eclipse-temurin-17-focal

WORKDIR /build

COPY search_index_article/src                    /build/ecom-mock/src
COPY search_index_article/pom.xml                /build/ecom-mock/pom.xml

COPY onfarming-mock/src                    /build/onfarming-mock/src
COPY onfarming-mock/pom.xml                /build/onfarming-mock/pom.xml

COPY pom.xml                    /build/pom.xml

# RUN mvn dependency:go-offline
RUN mvn install

WORKDIR /build

EXPOSE 6001 6002
