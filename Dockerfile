# Use the official Rust image for building
FROM rust:1-slim as builder

# Install all necessary system dependencies for tesseract and leptonica
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libleptonica-dev \
    libtesseract-dev \
    tesseract-ocr \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

# Create a new directory for our app
WORKDIR /usr/src/app

# Copy the project files into the container
COPY . .

# Build the application in release mode
# This will use the system dependencies we just installed
RUN cargo build --release

# --- Final Stage ---
# Use a minimal base image for a small final container
FROM debian:buster-slim

# Copy only the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/imagetotext /usr/local/bin/imagetotext

# Expose the port your application runs on. I will assume 8080.
# If your app uses a different port, we must change this.
EXPOSE 8080

# Set the command to run your application
CMD ["imagetotext"] 