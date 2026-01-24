FROM rust:1.63

# ——————————————————————————————————————————————————— INSTALLATION ——————————————————————————————————————————————————— #

COPY source .

ARG NETWORKLOOKUP_BEARERTOKEN
ARG NETWORKLOOKUP_ROUTER_DOMAIN
ARG NETWORKLOOKUP_DB_USER
ARG NETWORKLOOKUP_DB_PASSWORD

ENV NETWORKLOOKUP_BEARERTOKEN=${NETWORKLOOKUP_BEARERTOKEN}
ENV NETWORKLOOKUP_ROUTER_DOMAIN=${NETWORKLOOKUP_ROUTER_DOMAIN}
ENV NETWORKLOOKUP_DB_USER=${NETWORKLOOKUP_DB_USER}
ENV NETWORKLOOKUP_DB_PASSWORD=${NETWORKLOOKUP_DB_PASSWORD}

RUN cargo install --path .


# ————————————————————————————————————————————————————— RUNTIME ————————————————————————————————————————————————————— #

EXPOSE 8081

CMD ["NetworkLookup"]
