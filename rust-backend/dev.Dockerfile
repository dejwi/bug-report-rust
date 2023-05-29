# Using official rust base image
FROM rust:1.69.0

# Set the application directory
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y psmisc \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Install cargo-watch
RUN cargo install cargo-watch

RUN cargo install sqlx-cli

RUN rustup component add rustfmt
RUN cargo install sqlx-cli

ENV SQLX_OFFLINE true

RUN mkdir src
RUN echo 'fn main() { panic!("Dummy Image Called!")}' > ./src/main.rs
COPY ["Cargo.toml", "Cargo.lock",  "./"]
RUN cargo build
COPY ./ ./
#need to break the cargo cache
RUN touch ./src/main.rs
RUN cargo build

# Copy the files to the Docker image


EXPOSE 8000

CMD [ "cargo", "watch", "-x", "run"]

