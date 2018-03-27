FROM alpine:3.7

RUN apk add --no-cache curl openssl-dev rust
RUN apk add --no-cache --virtual .build-deps make git cargo
RUN mkdir /app
RUN cargo install --color never --root /app --git https://github.com/wehjin/ddnsimple.git --tag v0.2.0
RUN apk del .build-deps && rm -rf /root/.cargo

ENV DDNSIMPLE_SETTINGSFILE /app/settings.yaml
CMD /app/bin/ddnsimple

