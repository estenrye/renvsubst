FROM scratch
ARG $TARGETPLATFORM
ARG VERSION
COPY ./${$TARGETPLATFORM}/renvsubst /renvsubst
ENTRYPOINT ["/renvsubst"]
