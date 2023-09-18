FROM debian:stable-slim
WORKDIR /var/www/app

COPY pkg server

RUN addgroup --system server && \
	usermod -a -G server www-data && \
	chown -R www-data:server /var/www/app

USER www-data

EXPOSE 80

CMD ["./server/server"]
