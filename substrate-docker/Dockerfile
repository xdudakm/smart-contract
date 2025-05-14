FROM debian:bookworm-slim

COPY substrate-contracts-node /usr/local/bin/substrate-contracts-node

RUN chmod +x /usr/local/bin/substrate-contracts-node

EXPOSE 9944
EXPOSE 9933

ENTRYPOINT ["./usr/local/bin/substrate-contracts-node"]