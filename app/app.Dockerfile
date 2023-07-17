FROM rust:1.70

RUN cargo init app
WORKDIR /app

COPY  Cargo.toml  Cargo.lock /app/
# cache the dependencies
RUN cargo build

# Copy the rest of the project files to the working directory
COPY . /app

RUN cargo build

ENTRYPOINT ["cargo"]
