FROM rust:1.62.1 as zee-fetcher

RUN mkdir /home/zee-fetcher

WORKDIR /home/zee-fetcher

# Copy the project folder
COPY ./cronjobs ./cronjobs

COPY ./scripts ./scripts

COPY ./.env ./.env

RUN touch coinmarketcap.json

RUN cargo install --git https://github.com/ZeeFi/zee-oracle-fetcher.git

#RUN move cronjob  
RUN touch /etc/cron.d/zee_fetcher_cronjob && cp ./cronjobs/zee_fetcher_cronjob /etc/cron.d/zee_fetcher_cronjob


#Install Cron
RUN apt-get update
RUN apt-get -y install cron
RUN apt-get -y install --reinstall rsyslog


# Give execution rights on the cron scripts
RUN chmod +x ./scripts/zee_fetcher_script.sh

# Apply cron job
RUN crontab /etc/cron.d/zee_fetcher_cronjob


# Run the fetcher
CMD ["/bin/bash", "-c", "service rsyslog restart && ./scripts/zee_fetcher_script.sh && chmod 644 /etc/cron.d/zee_fetcher_cronjob && cron -f "]