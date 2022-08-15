FROM rust:1.62.1 as fetcher

RUN mkdir /home/fetcher

WORKDIR /home/fetcher

# Copy the project folder
COPY ./cronjobs ./cronjobs

COPY ./scripts ./scripts

COPY ./.env ./.env

RUN touch coinmarketcap.json

RUN cargo install --git https://github.com/valekar/fetcher.git

#RUN move cronjob  
RUN touch /etc/cron.d/fetcher_cronjob && cp ./cronjobs/fetcher_cronjob /etc/cron.d/fetcher_cronjob


#Install Cron
RUN apt-get update
RUN apt-get -y install cron
RUN apt-get -y install --reinstall rsyslog


# Give execution rights on the cron scripts
RUN chmod +x ./scripts/fetcher_script.sh

# Apply cron job
RUN crontab /etc/cron.d/fetcher_cronjob


# Run the fetcher
CMD ["/bin/bash", "-c", "service rsyslog restart && ./scripts/fetcher_script.sh && chmod 644 /etc/cron.d/fetcher_cronjob && cron -f "]