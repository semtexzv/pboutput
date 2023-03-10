FROM ubuntu:focal

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y wget gnupg
RUN echo "deb http://apt.postgresql.org/pub/repos/apt/ focal-pgdg main" >> /etc/apt/sources.list.d/pgdg.list
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -

RUN apt-get update && apt-get install -y curl git clang-10 llvm-10 clang gcc make build-essential libz-dev zlib1g-dev strace libssl-dev pkg-config

RUN apt-get update && apt-get install -y postgresql-11 postgresql-server-dev-11
RUN apt-get update && apt-get install -y postgresql-12 postgresql-server-dev-12
RUN apt-get update && apt-get install -y postgresql-13 postgresql-server-dev-13
RUN apt-get update && apt-get install -y postgresql-14 postgresql-server-dev-14
RUN apt-get update && apt-get install -y postgresql-15 postgresql-server-dev-15
RUN apt-get update && apt-get install -y ruby ruby-dev rubygems build-essential sudo
RUN gem install --no-document fpm

ARG USER=docker
ARG UID=1000
ARG GID=1000

RUN echo '%sudo ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
RUN echo "$USER ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/$USER
RUN useradd -G sudo -u ${UID} -m ${USER}
USER ${UID}:${GID}
WORKDIR /home/${USER}/pkg

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

ENV PATH="/home/${USER}/.cargo/bin:${PATH}"

RUN cargo install cargo-pgx
RUN cargo pgx init \
	--pg11=/usr/lib/postgresql/11/bin/pg_config \
	--pg12=/usr/lib/postgresql/12/bin/pg_config \
	--pg13=/usr/lib/postgresql/13/bin/pg_config \
	--pg14=/usr/lib/postgresql/14/bin/pg_config \
	--pg15=/usr/lib/postgresql/15/bin/pg_config

RUN sudo apt-get install -y protobuf-compiler xxd jq