from http.server import HTTPServer


from Dictionnary import createDictionnary
from handler import make_handler

# main start all requirement then launch the server
def main(server_class=HTTPServer, port=8000):
    DICTIONARY = createDictionnary()
    handler_class = make_handler(DICTIONARY)

    server_address = ("", port)
    httpd = server_class(server_address, handler_class)
    print(f"Server start on: {port}")

    httpd.serve_forever()


if __name__ == "__main__":
    main()