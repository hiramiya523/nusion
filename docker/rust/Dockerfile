FROM rust:1.72.0-slim-bullseye

ENV TZ Asia/Tokyo
# ARG UID=${FRONT_UID}
# ARG GID=${FRONT_GID}

ARG USERNAME=user
ARG GROUPNAME=user
ARG UID=1000
ARG GID=1000
RUN groupadd -g $GID $GROUPNAME && \
    useradd -m -s /bin/bash -u $UID -g $GID $USERNAME

# cargoの出力ディレクトリの指定
ENV CARGO_TARGET_DIR=/tmp/target \
  DEBIAN_FRONTEND=noninteractive \
  LC_CTYPE=ja_JP.utf8 \
  LANG=ja_JP.utf8

RUN apt-get update \
  && apt-get upgrade -y \
  && apt-get install -y -q --no-install-recommends\
  ca-certificates \
  locales \
  apt-transport-https\
  libssl-dev \
  libpq-dev \
  pkg-config \
  curl \
  build-essential \
  libdbus-1-dev \
  git \
  wget \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*
  # && echo "ja_JP UTF-8" > /etc/locale.gen \
  # && locale-gen \
  # && echo "install rust tools" \
  # && rustup component add rustfmt \
  # && cargo install cargo-watch cargo-make \
  # && cargo install sqlx-cli --no-default-features --features mysql

USER $USERNAME
WORKDIR /home/$USERNAME/app