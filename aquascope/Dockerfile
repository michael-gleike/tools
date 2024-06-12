FROM rust:latest

ENV BOOK_DIR=aquascope-book

RUN cargo install mdbook

WORKDIR /aquascope
RUN cargo install mdbook-aquascope --locked --version 0.3.1
RUN rustup toolchain install nightly-2023-08-25 -c rust-src rustc-dev llvm-tools-preview miri
RUN cargo +nightly-2023-08-25 install aquascope_front --git https://github.com/cognitive-engineering-lab/aquascope --tag v0.3.1 --locked
RUN cargo +nightly-2023-08-25 miri setup
RUN mdbook init "$BOOK_DIR"

WORKDIR "/aquascope/$BOOK_DIR"
RUN echo "" >> book.toml && \
    grep -q "/[preprocessor.aquascope]/d" book.toml || echo "#book.toml" >>book.toml && echo "[preprocessor.aquascope]" >> book.toml

WORKDIR /aquascope
COPY entrypoint.sh /aquascope/
ENTRYPOINT [ "/aquascope/entrypoint.sh" ]



# Install from Source: at the moment not fully functional and also slower
#FROM rust:latest

#ENV PNPM_HOME="/root/.local/bin"
#ENV BOOK_DIR=aquascope-book

#RUN cargo install mdbook
#RUN rustup toolchain install nightly-2023-08-25 -c rust-src rustc-dev llvm-tools-preview miri
#RUN curl https://get.pnpm.io/install.sh | bash
#RUN cargo install cargo-make --locked
#RUN cargo install depot-js --locked
#RUN apt-get update && apt-get install -y curl && apt-get clean
#RUN curl -sL https://deb.nodesource.com/setup_18.x | bash -  # Or adjust URL for desired version
#RUN apt-get update && apt-get install -y nodejs

#COPY entrypoint.sh /tool/
#WORKDIR /tool
#RUN git clone https://github.com/cognitive-engineering-lab/aquascope.git --tag v0.3.1
#RUN mv v0.3.1 aquascope
#WORKDIR /tool/aquascope
#RUN cargo make install-mdbook

#WORKDIR /tool
#RUN mdbook init "$BOOK_DIR"

#WORKDIR "/tool/$BOOK_DIR"
#RUN echo "" >> book.toml && \
#    grep -q "/[preprocessor.aquascope]/d" book.toml || echo "[preprocessor.aquascope]" >> book.toml

#WORKDIR /tool
#CMD [ "sh", "-c", "/tool/entrypoint.sh ${CMDARGS}" ]
