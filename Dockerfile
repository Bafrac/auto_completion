# Stage 1: Compiler le projet Rust
FROM rust:latest AS rust-builder

WORKDIR /build

# Copier le projet Rust
COPY rust/completion ./

# Compiler en release
RUN cargo build --release

# Stage 2: Image finale avec Python
FROM python:3.12-slim

WORKDIR /app

# Installer les dépendances système
RUN apt-get update && apt-get install -y --no-install-recommends bash && rm -rf /var/lib/apt/lists/*

# Copier les dictionnaires
COPY Dictionarys ./Dictionarys/

# Copier les fichiers Python
COPY python ./python/

# Copier l'exécutable Rust depuis le stage de build
COPY --from=rust-builder /build/target/release/completion ./

# Rendre l'exécutable Rust executable
RUN chmod +x ./completion

# Installer les dépendances Python (si requirements.txt existe)
RUN if [ -f python/requirements.txt ]; then pip install --no-cache-dir -r python/requirements.txt; fi

# Exposition du port si nécessaire
EXPOSE 8000
EXPOSE 8001

# Lancer Rust ou Python
CMD ["sh", "-c", "python /app/python/main.py"]

#CMD ["sh", "-c", "./completion"]