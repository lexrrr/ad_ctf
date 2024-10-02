#had problems with musl libc
FROM rustlang/rust:nightly
RUN apt-get -y update
RUN apt-get install -y sudo

# Create a seperate user and chown new directories if necessary
RUN addgroup --system service
RUN adduser --system --ingroup service --uid 1000 service

# Create our mapped data volume endpoint
RUN mkdir /data/

RUN sudo apt-get install -y python3.11 python3.11-dev python3-pip python3-venv
ENV PYTHONUNBUFFERED=1
#experimental:
#RUN chmod u+s /bin/su

# Copy our service
COPY . /service/

# Change the working directory.
WORKDIR /service/
RUN python3 -m venv .env
ENV PATH=".env/bin:$PATH"
RUN python3 -m pip install --upgrade pip
RUN python3 -m pip install -r requirements.txt
# Expose the service's port

#ENV MYSQL_PWD=userpass
#ENV PYTHONUNBUFFERED=TRUE
EXPOSE 2027


# not necessary because traffic can't be monitored
# RUN openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 3650 -nodes -subj "/C=DE/ST=Berlin/L=Berlin/O=TUB/OU=SecT/CN=Wonki"

#--release
RUN cargo +nightly build -r

# Run the service
ENTRYPOINT [ "cargo", "+nightly", "run", "-r"]
#ENTRYPOINT [ "cargo", "run", "--release"]