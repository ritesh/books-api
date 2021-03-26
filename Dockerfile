FROM public.ecr.aws/prima/rust:1.51.0-1 as builder

RUN USER=root cargo new --bin books-api
WORKDIR ./books-api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./
RUN rm ./target/release/deps/books_api*
RUN cargo build --release


FROM public.ecr.aws/spw-develop/mirror/essential/debian-buster-slim:latest
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3030

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /books-api/target/release/books-api ${APP}/books-api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./books-api"]
