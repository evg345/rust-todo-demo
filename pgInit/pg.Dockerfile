FROM postgres:12.12-alpine
COPY init.sql /docker-entrypoint-initdb.d/