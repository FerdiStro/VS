FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y wkhtmltopdf pandoc && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir -p /app
COPY build/input.html /app/input.html
COPY stop_server.sh /app/stop_server.sh

RUN chmod +x /app/stop_server.sh
RUN chmod +r /app/input.html

WORKDIR /app
RUN mkdir -p /app/output
RUN chmod 777 /app/output


CMD ["wkhtmltopdf", "--margin-top", "0", "--margin-bottom", "0", "--page-size", "A4", "input.html", "output/output.pdf"]
