FROM rust:1.65-alpine
WORKDIR /app


RUN apk update && apk add --virtual build-dependencies build-base gcc


COPY . .

WORKDIR src/master

RUN cargo install --path .

EXPOSE 3000

CMD ["master"]