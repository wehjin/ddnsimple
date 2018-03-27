FROM alpine:3.7

RUN apk add --no-cache curl make openssl-dev git rust cargo
