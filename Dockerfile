FROM rust:1.67

WORKDIR /usr/src/gsca-backend
COPY . .

RUN apt-get update && apt-get install -y default-libmysqlclient-dev

RUN cargo install diesel_cli --no-default-features --features mysql

RUN cargo install --path

ENV DB_URL=mysql://admin:admin@localhost/base

RUN echo "sed 's~DATABASE_URL=~${DB_URL}~' /usr/src/gsca-backend/.env.template > /usr/src/gsca-backend/.env" > /create_env.sh"
RUN chmod +x /app/create_env.sh && /app/create_env.sh
RUN diesel migration run

CMD ["gsca"] 

EXPOSE 8080

