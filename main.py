from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import urlparse, parse_qs
import json

DICTIONARY = [
    "cryptanalysis",
    "cryptographers",
    "cryptographic algorithm",
    "cryptography",
    "crypto",
    "crocodile",
    "crow",
    "cube"
]



def autocomplete(query: str, limit: int):
    if not query:
        return []
    query = query.lower()

    matches = [
        term for term in DICTIONARY
        if term.lower().startswith(query)
    ]

    # Tri alphabétique
    matches.sort(key=str.lower)

    # Limitation du nombre de résultats
    return matches[:limit]



class AutoCompleteHandler(BaseHTTPRequestHandler):

    def do_GET(self):
        parsed_url = urlparse(self.path)
        print(parsed_url)

        if parsed_url.path != "/autocomplete":
            self.send_error(404, "Not Found")
            return
        
        # add more protection for injection

        # get nbr result and the searched element
        params = parse_qs(parsed_url.query)
        query = params.get("query", [""])[0]
        limit = int(params.get("limit", [5])[0])

        result = autocomplete(query, limit)

        response = json.dumps(result, ensure_ascii=False)

        self.send_response(200)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.end_headers()
        self.wfile.write(response.encode("utf-8"))
        
        return 
    

def main(server_class=HTTPServer, handler_class=AutoCompleteHandler, port=8000):
    server_address = ("", port)
    httpd = server_class(server_address, handler_class)
    print(f"Serveur démarré sur le port {port}")
    httpd.serve_forever()


if __name__ == "__main__":
    main()