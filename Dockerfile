FROM rust

COPY . .

EXPOSE 8080:8080

RUN cargo build --release

CMD [ "./target/release/cn_simplified_to_traditional" ]