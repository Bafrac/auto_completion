# Auto-Compléteur



Ce programme est une API back-end complétant le début de mot donné en entrée.
Il lance un server HTTP et attend une requête et retourne une liste de mots contenant la chaine donnée.

## Prérequis

- Python ≥ 3.12


# Structure du projet

├── main.py # Point d’entrée du serveur HTTP
├── handler.py # Gestion des requêtes HTTP
├── createdictionary.py # Chargement du dictionnaire en mémoire
├── dictionarys/
│ ├── dictionary.txt
│ ├── dictionary_star_wars.txt
│ ├── dictionary_westeros.txt
│ ├── dictionary_star_trek.txt
│ └── dictionary_star_harry_potter.txt
└── README.md

# Lancement Docker

```bash
docker build -t autocomplete:latest .
docker run -p 8000:8000 autocomplete
```

## Lancement local
```bash
python3 main.py
```

# API
```bash
GET /autocomplete
```

### Paramètres
query   string
limit   int         optionnel

## Exemple d'utilisation
```bash
curl "http://127.0.0.1:8000/autocomplete?query=cr"
curl "http://127.0.0.1:8000/autocomplete?query=rar"
curl "http://127.0.0.1:8000/autocomplete?query=cry&limit=1"
```

```bash
 curl "http://127.0.0.1:8000/autocomplete?query=DE"
["dead", "dead", "deadly", "deadly", "deal"]
```
