from http.server import BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs

import json

# autocomplete search in dictionary and add to a result
# return à list
def autocomplete(query: str, limit: int, dictionary: list ):
    if not query:
        return []
    query = query.lower()

    matches = [
        term for term in dictionary
        if term.lower().startswith(query)
    ]

    matches.sort(key=str.lower)

    return matches[:limit]







# make_handler allow passing the dictionary between files
# return HTTP response to user 
def make_handler(dictionary):
    class AutoCompleteHandler(BaseHTTPRequestHandler):

        def do_GET(self):
            parsed_url = urlparse(self.path)
            print(parsed_url)

            if parsed_url.path != "/autocomplete":
                self.send_error(404, "Not Found")
                return
            
            # add more protection for injection

            
            params = parse_qs(parsed_url.query)
            query = params.get("query", [""])[0]
            limit = int(params.get("limit", [5])[0])

            result = autocomplete(query, limit, dictionary)

            response = json.dumps(result, ensure_ascii=False)

            self.send_response(200)
            self.send_header("Content-Type", "application/json; charset=utf-8")
            self.end_headers()
            self.wfile.write(response.encode("utf-8"))
            
            return 
    
    return AutoCompleteHandler