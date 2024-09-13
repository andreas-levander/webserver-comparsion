FROM golang:latest AS builder

ARG CGO_ENABLED=0
WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download
COPY . .

RUN go build

FROM scratch
COPY --from=builder /app/go-webserver-test /server
COPY --from=builder /app/data /data

ENV GIN_MODE=release
EXPOSE 8080

ENTRYPOINT ["/server"]