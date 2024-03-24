# Build frontend
FROM node:18-alpine3.14 as frontend
WORKDIR /app
COPY ./frontend /app
RUN yarn install
RUN yarn generate

# Build backend
FROM rust:1.77 as backend
WORKDIR /app
COPY ./backend /app
RUN cargo build --release
RUN cargo install --path .

# Build final image
FROM debian:12 as runtime
COPY --from=frontend /app/dist /usr/share/safehaven/static
COPY --from=backend /usr/local/cargo/bin/safehaven /bin/safehaven
ENV SAFEHAVEN_serve_public_path=/usr/share/safehaven/static
EXPOSE 28669
CMD ["/usr/local/bin/safehaven"]
