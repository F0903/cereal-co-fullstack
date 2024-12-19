FROM rust:1.83 AS build
WORKDIR /backend
COPY .docker.env .env
COPY . .
RUN cargo build -r


FROM ubuntu:24.04
RUN apt-get update && apt-get install -y

# Copy exe
COPY --from=build /backend/target/release/cereal_co_backend /backend/build/
# Copy static files
COPY --from=build /backend/static/ /backend/build/static/


WORKDIR /backend/build/
RUN ls -la
ENV DOCKER=true
ENV DB_PROTOCOL=mysql
ENV DB_USER=root
ENV DB_PASSWORD=root
ENV DB_HOST=localhost
ENV DB_PORT=3306
ENV DB_DATABASE=week10
EXPOSE 8000
ENTRYPOINT ./cereal_co_backend