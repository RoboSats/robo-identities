FROM debian:testing-slim AS base

# install common packages
RUN apt-get update \
    && apt-get install -y \
        wget \
        curl \
        unzip \
        build-essential \
        openjdk-17-jre-headless \
        --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

# Set an environment variable for convenience.
ENV GRADLE_ROOT=${HOME}/opt/gradle

RUN mkdir -p ${GRADLE_ROOT}
RUN wget https://services.gradle.org/distributions/gradle-7.5.1-bin.zip -O gradle-7.5.1-bin.zip \
    && sha256sum gradle-7.5.1-bin.zip \
    && echo "f6b8596b10cce501591e92f229816aa4046424f3b24d771751b06779d58c8ec4  gradle-7.5.1-bin.zip" | sha256sum -c - \
    && unzip gradle-7.5.1-bin.zip -d ${GRADLE_ROOT} \
    && rm gradle-7.5.1-bin.zip

# Add the relevant directories to the $PATH.
ENV PATH=${PATH}:${GRADLE_ROOT}/gradle-7.5.1/bin

# Set the ${ANDROID_HOME} variable, so that the tools can find our installation.
# See https://developer.android.com/studio/command-line/variables#envar.
ENV ANDROID_HOME=${HOME}/opt/android-sdk

# Download and extract the command-line tools into ${ANDROID_HOME}.
RUN mkdir -p ${ANDROID_HOME}
RUN wget https://dl.google.com/android/repository/commandlinetools-linux-8512546_latest.zip \
        -O commandlinetools-linux-8512546_latest.zip \
    && sha256sum commandlinetools-linux-8512546_latest.zip \
    && echo "2ccbda4302db862a28ada25aa7425d99dce9462046003c1714b059b5c47970d8 commandlinetools-linux-8512546_latest.zip" | sha256sum -c - \
    && unzip commandlinetools-linux-8512546_latest.zip -d ${ANDROID_HOME}/cmdline-tools \
    && rm commandlinetools-linux-8512546_latest.zip

# Add the relevant directories to the $PATH.
ENV PATH=${PATH}:${ANDROID_HOME}/cmdline-tools/cmdline-tools/bin:${ANDROID_HOME}/platform-tools

RUN yes | sdkmanager --licenses \
    && sdkmanager --verbose \
        "build-tools;30.0.3" \
        "ndk;25.1.8937393" \
        "platforms;android-33"
ENV NDK_HOME=${ANDROID_HOME}/ndk/25.1.8937393

# install rust tools
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain nightly -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add \
        aarch64-linux-android \
        armv7-linux-androideabi \
        x86_64-linux-android \
    && rustup toolchain install nightly \
    && rustup target add --toolchain nightly \
        aarch64-linux-android \
        armv7-linux-androideabi \
        x86_64-linux-android 

ENV PATH="${PATH}:${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin"

# install cargo tools
RUN cargo install cargo-ndk 

COPY . /root

# Robonames
FROM base AS robonames
WORKDIR /root/robonames

# Robohash
FROM base AS robohash
WORKDIR /root/robohash

