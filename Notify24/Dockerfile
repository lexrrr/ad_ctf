FROM gradle:jdk17-focal AS builder
WORKDIR /app

COPY build.gradle.kts /app/
COPY settings.gradle.kts /app/

RUN gradle compileJava

COPY src /app/src
RUN gradle build

FROM amazoncorretto:17-alpine AS runtime

RUN adduser -S -D app
RUN addgroup -S app
RUN addgroup app app

USER app
COPY --from=builder --chown=app:app /app/build/libs/GRPC-Kotlin-0.0.1-SNAPSHOT.jar /app/service.jar

EXPOSE 6060
CMD java -jar /app/service.jar