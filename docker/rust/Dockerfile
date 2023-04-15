FROM rust:1.68.0

ENV USER_ID=1000 \
    USER=appuser

# ユーザーを作成
RUN  useradd -m -u ${USER_ID} appuser \
    && cargo install cargo-watch cargo-make

WORKDIR /work/backend/server
EXPOSE 8080

CMD ["cargo", "watch", "-x", "run"]
