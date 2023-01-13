# [Choice]Rumbas version
ARG VARIANT=local
FROM m8rex/rumbas:${VARIANT}

# Install needed packages, yarn, nvm and setup non-root user. Use a separate RUN statement to add your own dependencies.
ARG USERNAME=rumbas
ARG USER_UID=1000

RUN adduser -D -u ${USER_UID} ${USERNAME}
USER rumbas