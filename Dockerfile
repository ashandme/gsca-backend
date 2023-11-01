FROM rust:1.73

WORKDIR /usr/src/gsca-backend
COPY . .

RUN apt-get update && apt-get install -y default-libmysqlclient-dev

RUN cargo install diesel_cli --no-default-features --features mysql

RUN cargo install --path .

ENV DATABASE_URL=mysql://admin:admin@localhost/base

CMD bash -c "diesel setup && gsca" 

EXPOSE 3000

