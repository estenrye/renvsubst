FROM scratch
ARG TARGET
ARG VERSION
COPY ./${TARGET}/renvsubst /renvsubst
ENTRYPOINT ["/renvsubst"]
