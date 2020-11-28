FROM postgres
ENV POSTGRES_PASSWORD postgres 
ENV POSTGRES_DB postgres 
COPY db.sql /docker-entrypoint-initdb.d/
RUN echo "host all  all    0.0.0.0/0  md5" >> /etc/postgresql/postgresql.conf
RUN echo "listen_addresses='*'" >> /etc/postgresql/postgresql.conf