FROM rust:1.47 as builder

WORKDIR /fair-api
COPY Cargo.toml .
COPY src/ src
RUN cargo build --release

FROM debian:buster-slim

ARG APP=/usr/src/app
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata curl net-tools\
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ENV TZ=Etc/UTC APP_USER=appuser
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /fair-api/target/release/fair-api ${APP}/fair-api

RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}

CMD ["./fair-api"]