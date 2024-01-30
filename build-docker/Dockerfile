FROM ubuntu:latest

# Install benötigte Pakete
RUN apt-get update && \
    apt-get install -y wkhtmltopdf pandoc && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*


COPY input.html /app/input.html
RUN chmod +r /app/input.html


WORKDIR /app
RUN mkdir -p /app/output

RUN chmod 755 /app/output



CMD ["wkhtmltopdf", "--margin-top", "0", "--margin-bottom", "0","--page-size", "A4", "input.html", "output/output.pdf"]


